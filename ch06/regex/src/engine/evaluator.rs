//! 命令列と入力文字列を受け取り、マッチングを行う
use super::Instruction;
use crate::helper::safe_add;
use std::{
    collections::VecDeque,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverFlow,
    SPOverFlow,
    InvalidPC,
    InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

/// 命令列の評価を行う関数。
///
/// instが命令列となり、その命令列を用いて入力文字列lineにマッチさせる。
/// offsetにはlineが行頭から何文字目かの数字を渡す。
/// is_depthがtrueの場合に深さ優先探索を、falseの場合に幅優先探索を行う。
///
/// 実行時エラーが起きた場合はErrを返す。
/// マッチ成功時はOk(true)を、失敗時はOk(false)を返す。
pub fn eval(inst: &[Instruction], line: &[char], offset: usize, is_depth: bool) -> Result<bool, EvalError> {
    if is_depth {
        eval_depth(inst, line, offset, 0, 0)
    } else {
        eval_width(inst, line, offset)
    }
}

/// 深さ優先探索で再帰的にマッチングを行う評価器
fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    offset: usize,
    mut pc: usize,
    mut sp: usize,
) -> Result<bool, EvalError> {
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };

        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Dot => {
                if let Some(_) = line.get(sp) {
                    safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                    safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                } else {
                    return Ok(false);
                }
            }
            Instruction::Caret => {
                if offset == 0 && sp == 0 {
                    safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                } else {
                    return Ok(false);
                }
            }
            Instruction::Dollar => {
                if sp == line.len() {
                    safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                } else {
                    return Ok(false);
                }
            }
            Instruction::Match => {
                return Ok(true);
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Split(addr1, addr2) => {
                if eval_depth(inst, line, offset, *addr1, sp)? || eval_depth(inst, line, offset, *addr2, sp)? {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
}

/// 幅優先探索で再帰的にマッチングを行う評価器
fn eval_width(inst: &[Instruction], line: &[char], offset: usize) -> Result<bool, EvalError> {
    let mut queue: VecDeque<(&Instruction, usize, usize)> = VecDeque::new();
    let next = if let Some(i) = inst.get(0) {
        i
    } else {
        return Err(EvalError::InvalidContext); // ここのエラーはどうするか迷った
    };
    queue.push_back((next, 0, 0));
    let mut result = false;

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        match node.0 {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(node.2) {
                    if c == sp_c {
                        let mut pc = node.1;
                        let mut sp = node.2;
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                        let next = if let Some(i) = inst.get(pc) {
                            i
                        } else {
                            return Err(EvalError::InvalidContext);
                        };
                        queue.push_back((next, pc, sp));
                    }
                }
            }
            Instruction::Dot => {
                if let Some(_) = line.get(node.2) {
                    let mut pc = node.1;
                    let mut sp = node.2;
                    safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                    safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    let next = if let Some(i) = inst.get(pc) {
                        i
                    } else {
                        return Err(EvalError::InvalidContext);
                    };
                    queue.push_back((next, pc, sp));
                }
            }
            Instruction::Caret => {
                if offset == 0 && node.2 == 0 {
                    let mut pc = node.1;
                    safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                    let next = if let Some(i) = inst.get(pc) {
                        i
                    } else {
                        return Err(EvalError::InvalidContext);
                    };
                    queue.push_back((next, pc, node.2));
                }
            }
            Instruction::Dollar => {
                if node.2 == line.len() {
                    let mut pc = node.1;
                    safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                    let next = if let Some(i) = inst.get(pc) {
                        i
                    } else {
                        return Err(EvalError::InvalidContext);
                    };
                    queue.push_back((next, pc, node.2));
                }
            }
            Instruction::Match => {
                result = true;
                break;
            }
            Instruction::Jump(addr) => {
                let next = if let Some(i) = inst.get(*addr) {
                    i
                } else {
                    return Err(EvalError::InvalidContext);
                };
                queue.push_back((next, *addr, node.2));
            }
            Instruction::Split(addr1, addr2) => {
                let next1 = if let Some(i) = inst.get(*addr1) {
                    i
                } else {
                    return Err(EvalError::InvalidContext);
                };
                let next2 = if let Some(i) = inst.get(*addr2) {
                    i
                } else {
                    return Err(EvalError::InvalidContext);
                };
                queue.push_back((next1, *addr1, node.2));
                queue.push_back((next2, *addr2, node.2));
            }
        }
    };
    return Ok(result);
}
