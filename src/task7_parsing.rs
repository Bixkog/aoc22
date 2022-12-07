use itertools::Itertools;

use nom::{
  IResult,
  bytes::complete::{tag, take_until},
  character::complete::{space1, digit1, alpha1},
  branch::alt,
  combinator::{iterator, all_consuming},
};

#[derive(Debug)]
pub enum LsRow {
    File {file_size: u64, file_name: String},
    Dir {dir_name: String}
}

#[derive(Debug)]
pub enum Command {
    Cd{arg: String},
    Ls{output: Vec<LsRow>}
}

pub fn parse_commands(i: &str) -> IResult<&str, Vec<Command>> {
    all_consuming(p_commands)(i)
}

fn p_commands(i: &str) -> IResult<&str, Vec<Command>> {
    let mut it = iterator(i, p_command);
    let commands = it.collect_vec();
    let (i, _) = it.finish()?;
    return Ok((i, commands))
}

fn p_command(i: &str) -> IResult<&str, Command> {
    alt((p_cd, p_ls))(i)
}

fn p_cd(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ cd ")(i)?;
    let (i, target_dir) = alt((alpha1, tag("/"), tag("..")))(i)?;
    let (i, _) = tag("\r\n")(i)?;

    Ok((i, Command::Cd{arg: target_dir.to_string()}))
}

fn p_ls(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ls")(i)?;
    let (i, _) = tag("\r\n")(i)?;
    let mut it = iterator(i, p_ls_row);
    let ls = Command::Ls{output: it.collect_vec()};
    let (i, _) = it.finish()?;
    return Ok((i, ls))
}

fn p_ls_row(i: &str) -> IResult<&str, LsRow> {
    alt((p_ls_row_file, p_ls_row_dir))(i)
}

fn p_ls_row_file(i: &str) -> IResult<&str, LsRow> {
    let (i, file_size) = digit1(i)?;
    let (i, _) = space1(i)?;
    let (i, file_name) = take_until("\r\n")(i)?;
    let (i, _) = tag("\r\n")(i)?;

    Ok((i, LsRow::File { file_size: file_size.to_string().parse().unwrap(), file_name: file_name.to_string() }))
}

fn p_ls_row_dir(i: &str) -> IResult<&str, LsRow> {
    let (i, _) = tag("dir ")(i)?;
    let (i, dir_name) = alpha1(i)?;
    let (i, _) = tag("\r\n")(i)?;

    Ok((i, LsRow::Dir { dir_name: dir_name.to_string() }))
}

