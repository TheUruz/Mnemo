use std::error::Error;
use brush_parser::{tokenize_str, parse_tokens, ParserOptions, SourceInfo, ast::{Command, SimpleCommand, CompoundCommand, CompoundList, AndOrList, Pipeline, AndOr, Program}};

pub fn collect_from_program(input: &str) -> Result<Vec<SimpleCommand>, Box<dyn Error>> {
    let program = parse_shell_to_program(input)?;
    let mut out: Vec<SimpleCommand> = Vec::new();
    for stmt in &program.complete_commands {
            collect_from_compound_list(stmt, &mut out);
    }
    return Ok(out);
}

fn parse_shell_to_program(input: &str) -> Result<Program, Box<dyn Error>> {
    let tokens = tokenize_str(input)?;
    let source_info = SourceInfo::default();
    let program = parse_tokens(&tokens, &ParserOptions::default(), &source_info)?;
    Ok(program)
}

fn collect_from_compound_list(list: &CompoundList, out: &mut Vec<SimpleCommand>) {
    for element in &list.0 {
        collect_from_and_or(&element.0, out);
    }
}

fn collect_from_and_or(and_or: &AndOrList, out: &mut Vec<SimpleCommand>) {
    collect_from_pipeline(&and_or.first, out);
    for item in &and_or.additional {
        match item {
            AndOr::And(p) | AndOr::Or(p) => {
                collect_from_pipeline(p, out);
            }
        }
    }
}

fn collect_from_pipeline(pipeline: &Pipeline, out: &mut Vec<SimpleCommand>) {
    for command in &pipeline.seq {
        collect_from_command(command, out);
    }
}

fn collect_from_command(cmd: &Command, out: &mut Vec<SimpleCommand>) {
    match cmd {
        Command::Simple(simple) => {
            out.push(simple.clone());
        }
        Command::Compound(compound, _redirects) => {
            match compound {
                CompoundCommand::Subshell(subshell) => {
                    collect_from_compound_list(&subshell.list, out);
                }
                _ => {}
            }
        }
        _ => {}
    }
}