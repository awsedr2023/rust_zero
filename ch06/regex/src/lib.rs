//! # 正規表現エンジン用クレート。
//!
//! ## 利用例
//!
//! ```
//! use my_regex;
//! let expr = "^a(bc)+|c(def)*$"; // 正規表現
//! let line = "cdefdefdef"; // マッチ対象文字列
//! my_regex::do_matching(expr, line, 0, true); // 幅優先探索でマッチング
//! my_regex::print(expr); // 正規表現のASTと命令列を表示
//! ```
mod engine;
mod helper;

pub use engine::{do_matching, print};
