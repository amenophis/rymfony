use clap::App;
use clap::SubCommand;
use prettytable::format;
use prettytable::Table;

use crate::php;

pub(crate) fn command_config<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("php:list").about("List all available PHP executables")
}

pub(crate) fn php_list() {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top],
            format::LineSeparator::new('─', '┬', '┌', '┐'),
        )
        .separators(
            &[format::LinePosition::Bottom],
            format::LineSeparator::new('─', '┴', '└', '┘'),
        )
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('─', '┼', '├', '┤'),
        )
        .padding(1, 1)
        .build();

    table.set_format(format);
    table.set_titles(row!["Binary path"]);

    for binary in php::binaries::all() {
        table.add_row(row![binary]);
    }

    table.printstd();
}
