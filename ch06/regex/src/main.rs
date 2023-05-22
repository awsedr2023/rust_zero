mod engine;
mod helper;

use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), DynError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }

    Ok(())
}

/// ファイルをオープンし、行ごとにマッチングを行う。
///
/// マッチングはそれぞれの行頭から1文字ずつずらして行い、
/// いずれかにマッチした場合に、その行がマッチしたもとみなす。
///
/// たとえば、abcdという文字列があった場合、以下の順にマッチが行われ、
/// このいずれかにマッチした場合、与えられた正規表現にマッチする行と判定する。
///
/// - abcd
/// - bcd
/// - cd
/// - d
fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    engine::print(expr)?;
    println!();

    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], i, true)? {
                println!("{line}");
                break;
            }
        }
    }

    Ok(())
}

// 単体テスト。プライベート関数もテスト可能
#[cfg(test)]
mod tests {
    use crate::{
        engine::do_matching,
        helper::{safe_add, SafeAdd},
    };

    #[test]
    fn test_safe_add() {
        let n: usize = 10;
        assert_eq!(Some(30), n.safe_add(&20));

        let n: usize = !0; // 2^64 - 1 (64 bits CPU)
        assert_eq!(None, n.safe_add(&1));

        let mut n: usize = 10;
        assert!(safe_add(&mut n, &20, || ()).is_ok());

        let mut n: usize = !0;
        assert!(safe_add(&mut n, &1, || ()).is_err());
    }

    #[test]
    fn test_matching_depth() {
        // パースエラー
        assert!(do_matching("+b", "bbb", 0, true).is_err());
        assert!(do_matching("*b", "bbb", 0, true).is_err());
        assert!(do_matching("|b", "bbb", 0, true).is_err());
        assert!(do_matching("?b", "bbb", 0, true).is_err());

        // パース成功、マッチ成功
        assert!(do_matching("abc|def", "def", 0, true).unwrap());
        assert!(do_matching("(abc)*", "abcabc", 0, true).unwrap());
        assert!(do_matching("(ab|cd)+", "abcdcd", 0, true).unwrap());
        assert!(do_matching("abc?", "ab", 0, true).unwrap());
        assert!(do_matching("((((a*)*)*)*)", "aaaaaaaaa", 0, true).unwrap());
        assert!(do_matching("(a*)*b", "aaaaaaaaab", 0, true).unwrap());
        assert!(do_matching("(a*)*b", "b", 0, true).unwrap());
        assert!(do_matching("a**b", "aaaaaaaaab", 0, true).unwrap());
        assert!(do_matching("a**b", "b", 0, true).unwrap());
        assert!(do_matching("a.c", "abc", 0, true).unwrap());
        assert!(do_matching("^abc", "abcdef", 0, true).unwrap());
        assert!(do_matching("abc$", "abc", 0, true).unwrap());

        // パース成功、マッチ失敗
        assert!(!do_matching("abc|def", "efa", 0, true).unwrap());
        assert!(!do_matching("(ab|cd)+", "", 0, true).unwrap());
        assert!(!do_matching("abc?", "acb", 0, true).unwrap());
        assert!(!do_matching("a.c", "ac", 0, true).unwrap());
        assert!(!do_matching("^abc", "defabc", 0, true).unwrap());
        assert!(!do_matching("^abc", "abcdef", 1, true).unwrap());
        assert!(!do_matching("abc$", "abcdef", 0, true).unwrap());
    }

    #[test]
    fn test_matching_width() {
        // パースエラー
        assert!(do_matching("+b", "bbb", 0, false).is_err());
        assert!(do_matching("*b", "bbb", 0, false).is_err());
        assert!(do_matching("|b", "bbb", 0, false).is_err());
        assert!(do_matching("?b", "bbb", 0, false).is_err());

        // パース成功、マッチ成功
        assert!(do_matching("abc|def", "def", 0, false).unwrap());
        assert!(do_matching("(abc)*", "abcabc", 0, false).unwrap());
        assert!(do_matching("(ab|cd)+", "abcdcd", 0, false).unwrap());
        assert!(do_matching("abc?", "ab", 0, false).unwrap());
        assert!(do_matching("((((a*)*)*)*)", "aaaaaaaaa", 0, false).unwrap());
        assert!(do_matching("(a*)*b", "aaaaaaaaab", 0, false).unwrap());
        assert!(do_matching("(a*)*b", "b", 0, false).unwrap());
        assert!(do_matching("a**b", "aaaaaaaaab", 0, false).unwrap());
        assert!(do_matching("a**b", "b", 0, false).unwrap());
        assert!(do_matching("a.c", "abc", 0, false).unwrap());
        assert!(do_matching("^abc", "abcdef", 0, false).unwrap());
        assert!(do_matching("abc$", "abc", 0, false).unwrap());

        // パース成功、マッチ失敗
        assert!(!do_matching("abc|def", "efa", 0, false).unwrap());
        assert!(!do_matching("(ab|cd)+", "", 0, false).unwrap());
        assert!(!do_matching("abc?", "acb", 0, false).unwrap());
        assert!(!do_matching("a.c", "ac", 0, false).unwrap());
        assert!(!do_matching("^abc", "defabc", 0, false).unwrap());
        assert!(!do_matching("^abc", "abcdef", 1, false).unwrap());
        assert!(!do_matching("abc$", "abcdef", 0, false).unwrap());
    }
}
