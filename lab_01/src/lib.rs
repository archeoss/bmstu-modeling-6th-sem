#[macro_use]
extern crate prettytable;
use constants::*;
use prettytable::{color, format, Attr, Cell, Row, Table};
pub mod constants;
mod euler;
mod picard;

//(u^2 + x)u' = 1
//u(1) = 0
pub fn task1() {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Title],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_titles(row![FdBmb=>
        " Arg. ",
        " Analytic ",
        " Euler ",
        " 1-st Pikar ",
        " 2-nd ",
        " 3-rd ",
        " 4-th ",
    ]);

    let args = (0..STEPS)
        .map(|step| SIZE.mul_add(step as f64, TASK1_U0)) // a.mul_add(b, c) == a * b + c
        .collect::<Vec<f64>>();
    let (analytic, euler, picard1, picard2, picard3, picard4) = (
        args.iter()
            .map(|&arg| TASK1_ANALYTIC(arg))
            .collect::<Vec<f64>>(),
        euler::solve(TASK1_EULER, TASK1_U0, TASK1_X0),
        picard::solve(TASK1_PICARD1, TASK1_U0, TASK1_X0),
        picard::solve(TASK1_PICARD2, TASK1_U0, TASK1_X0),
        picard::solve(TASK1_PICARD3, TASK1_U0, TASK1_X0),
        picard::solve(TASK1_PICARD4, TASK1_U0, TASK1_X0),
    );
    for (i, arg) in args.iter().enumerate() {
        table.add_row(Row::new(
            vec![
                arg,
                &analytic[i],
                &euler[i],
                &picard1[i],
                &picard2[i],
                &picard3[i],
                &picard4[i],
            ]
            .iter()
            .map(|&res| {
                Cell::new(format!(" {res:6.2} ").as_str())
                    .with_style(Attr::BackgroundColor(color::BLUE))
                    .with_style(Attr::ForegroundColor(color::BLACK))
                    .with_style(Attr::Italic(true))
            })
            .collect(),
        ));
    }
    table.printstd();
}

//1 - 2xuu' = u^3u'
//u(0.5) = 0
pub fn task2() {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Title],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_titles(row![FdBmb=>
        " Arg. ",
        " Analytic ",
        " Euler ",
        " 1-st Pikar ",
        " 2-nd ",
        " 3-rd ",
        " 4-th ",
    ]);

    let args = (0..STEPS)
        .map(|step| SIZE.mul_add(step as f64, TASK2_U0)) // a.mul_add(b, c) == a * b + c
        .collect::<Vec<f64>>();
    let (analytic, euler, picard1, picard2, picard3, picard4) = (
        args.iter()
            .map(|&arg| TASK2_ANALYTIC(arg))
            .collect::<Vec<f64>>(),
        euler::solve(TASK2_EULER, TASK2_U0, TASK2_X0),
        picard::solve(TASK2_PICARD1, TASK2_U0, TASK2_X0),
        picard::solve(TASK2_PICARD2, TASK2_U0, TASK2_X0),
        picard::solve(TASK2_PICARD3, TASK2_U0, TASK2_X0),
        picard::solve(TASK2_PICARD4, TASK2_U0, TASK2_X0),
    );
    for (i, arg) in args.iter().enumerate() {
        table.add_row(Row::new(
            vec![
                arg,
                &analytic[i],
                &euler[i],
                &picard1[i],
                &picard2[i],
                &picard3[i],
                &picard4[i],
            ]
            .iter()
            .map(|&res| {
                Cell::new(format!(" {res:6.2} ").as_str())
                    .with_style(Attr::BackgroundColor(color::BLUE))
                    .with_style(Attr::ForegroundColor(color::BLACK))
                    .with_style(Attr::Italic(true))
            })
            .collect(),
        ));
    }
    // println!("{picard1:?}");
    table.printstd();
}

//u'(x) = u^2 + x^2
//u(0) = 0
pub fn task3() {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Title],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_titles(row![FdBmb=>
        " Arg. ",
        " Euler ",
        " 1-st Pikar ",
        " 2-nd ",
        " 3-rd ",
        " 4-th ",
    ]);

    let args = (0..STEPS)
        .map(|step| SIZE.mul_add(step as f64, TASK3_U0)) // a.mul_add(b, c) == a * b + c
        .collect::<Vec<f64>>();
    let (euler, picard1, picard2, picard3, picard4) = (
        euler::solve(TASK3_EULER, TASK3_U0, TASK3_X0),
        picard::solve(TASK3_PICARD1, TASK3_U0, TASK3_X0),
        picard::solve(TASK3_PICARD2, TASK3_U0, TASK3_X0),
        picard::solve(TASK3_PICARD3, TASK3_U0, TASK3_X0),
        picard::solve(TASK3_PICARD4, TASK3_U0, TASK3_X0),
    );
    for (i, arg) in args.iter().enumerate() {
        table.add_row(Row::new(
            vec![
                arg,
                &euler[i],
                &picard1[i],
                &picard2[i],
                &picard3[i],
                &picard4[i],
            ]
            .iter()
            .map(|&res| {
                Cell::new(format!(" {res:6.2} ").as_str())
                    .with_style(Attr::BackgroundColor(color::BLUE))
                    .with_style(Attr::ForegroundColor(color::BLACK))
                    .with_style(Attr::Italic(true))
            })
            .collect(),
        ));
    }
    // println!("{picard1:?}");
    table.printstd();
}

#[test]
fn task1_analyt() {
    assert_eq!(TASK1_ANALYTIC(0.0), 1.0);
}

#[test]
fn task2_analyt() {
    assert_eq!(TASK2_ANALYTIC(0.0), 0.5);
}
