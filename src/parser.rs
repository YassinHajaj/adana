use crate::prelude::*;

pub enum CacheCommand<'a> {
    Add {
        aliases: Vec<&'a str>,
        value: &'a str,
    },
    List,
    Remove(&'a str),
    Get(&'a str),
    Exec(&'a str),
    Using(&'a str),
    Dump(Option<&'a str>),
}

fn add_command(command: &str) -> Res<CacheCommand> {
    map(
        pair(
            preceded(
                preceded(multispace0, tag_no_case("ADD")),
                many0(preceded(
                    preceded(multispace1, tag_no_case("-a")),
                    preceded(
                        multispace1,
                        cut(verify(
                            take_while(|c: char| c.is_alphanumeric()),
                            |s: &str| !s.is_empty(),
                        )),
                    ),
                )),
            ),
            preceded(
                multispace1,
                cut(verify(rest.map(|s: &str| s.trim()), |s: &str| {
                    !s.is_empty()
                })),
            ),
        ),
        |(aliases, value)| CacheCommand::Add { aliases, value },
    )(command)
}
fn del_command(command: &str) -> Res<CacheCommand> {
    map(
        alt((
            extract_key(tag_no_case("del")),
            extract_key(tag_no_case("delete")),
        )),
        CacheCommand::Remove,
    )(command)
}
fn get_command(command: &str) -> Res<CacheCommand> {
    map(extract_key(tag_no_case("GET")), CacheCommand::Get)(command)
}
fn exec_command(command: &str) -> Res<CacheCommand> {
    map(extract_key(tag_no_case("EXEC")), CacheCommand::Exec)(command)
}

fn list_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            alt((tag_no_case("LIST"), tag("ls"))),
            cut(verify(rest, |s: &str| s.trim().is_empty() || s == "\n")),
        ),
        |_| CacheCommand::List,
    )(command)
}

fn extract_key<'a, F>(parser: F) -> impl Fn(&'a str) -> Res<&'a str>
where
    F: Fn(&'a str) -> Res<&'a str>,
{
    move |s: &str| {
        preceded(
            &parser,
            preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ),
        )(s)
    }
}

fn using_command(command: &str) -> Res<CacheCommand> {
    map(
        alt((
            extract_key(tag_no_case("USING")),
            extract_key(tag_no_case("USE")),
        )),
        CacheCommand::Using,
    )(command)
}

fn dump_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case("DUMP"),
            cut(verify(rest, |s: &str| {
                s.is_empty() || s.starts_with(' ') || s == "\n"
            }))
            .and_then(opt(preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ))),
        ),
        CacheCommand::Dump,
    )(command)
}

pub fn parse_command(command: &str) -> Res<CacheCommand> {
    preceded(
        multispace0,
        alt((
            add_command,
            del_command,
            get_command,
            using_command,
            dump_command,
            list_command,
            exec_command,
        )),
    )(command)
}
