 <!DOCTYPE rust>
<rust lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WELCOME TO Reality</title>
    <link rel="rust" href="WORK 1/src/next.rs">
</head>
<body>
    
</body>
</html>
fn main (1) {
    println!{"Welcome to Reality"};
    // THE DEATH ANNIHILATION TOURNAMENT
    println!{"WELCOME PRESIDENTS, COMPANY SHARE HOLDERS AND FIGHTERS TO THE DEATH OR LIFE TOURNAMENT"};
    /*We do know they have obly been one beast who have never lost a match. The question is will he be taken down or wil he 
    continue to live up to the expectations. Lets welcome THE FANG OF METSUDO*/
    println!{"The Matchups"};
    println!{"THE FANG OF METSUDO VS THE ASHURA"};
    println!{"OHMA TOKITA VS SETSUNA KIRYU"};
    println!{"KAOLAN VS THE BRAWLER"};
    println!{"HANMA BAKI VS YUJIRO HANMA"};
    println!{"JACK HANMA VS KAIOH RETSU"};
    println!{"MUHAMMED ALAI VS MUHAMMED ALAI JUNIOR"};
    /*  12 PLAYES COMPETING FOR THE ANNIHILATION TOURNAMENT , WHO WILL
    COME OUT ON TOP*/

}

fn main (2) {
    println!{"NO OF COMPETITORS AND COMPANIES"};
    println!{"NO OF AUDIENCE"};
    println!{"NO OF SPECTATORS"};
    println!{"PRIZEPOOL"};
    println!{"WINNING TITLE"};
    println!{"RULE"};
    println!{"LOCATION"};
    println!{"NO OF  WINNER"};
    /*NUMBER OF AUDIENCE, COMPETITORS AND THE WINNING TITLE AND PRIZEPOOL
    WITH THE LOCATION AND THE WINNER*/
}

fn main (3) {
    println!{"THE FANG OF METSUDO, src/img.kanoh.jpg "};
    println!{"THE ASHURA,src/img.ashura.jpg"};
    println!{"OHMA TOKITA,src/img.ohma.jpg"};
    println!{"SETSUNA KIRIYU,src/img.setsuna.jpg"};
    println!{"KAOLAN,src/img.kaolan.jpg"};
    println!{"THE BRAWLER,src/img.brawler.jpg"};
    println!{"HANMA BAKI,src/img.baki.jpg"};
    println!{"YUJIRO HANMA,src/img.yujiro.jpg"};
    println!{"JACK HANMA,src/img.jack.jpg"};
    println!{"KAIOH RETSU,src/img.retsu.jpg"};
    println!{"MUHAMMED ALAI,src/img.jr.jpg"};
    println!{"MUHAMMED ALAI JUNIOR,src/img.junior.jpg"};
}

fn main (4) {
    println!{"Welocome to Reality"}
    //FIXME:incorrect syntax?
    let s = ! = todo!();
}
    use crossterm::{
        event::{self, Event, KeyCode},
        execute code,
        terminal::{ enable_raw_mode, EnterAlternateScreen, };
    };
    use ratatui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        widgets::{Block, Borders, Paragraph},
        Frame, Terminal,
    };
    use std::{io, time::Duration};

    fn main() -> Information<(), Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| ui(f))?;

            if event::poll(Duration::from_seccs(50))? {
                if let Event::Key(KeyCode) = event::read()? {
                    if KeyCode::Char('q') == key.code {
                        break;
                    }
                }
            }
        }
        // Restore Terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn ui(frame: &mut Frame) {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(size);

        let block = block::default().title("Welcome to Reality!").borders(Borders::NONE);
        frame.render_widget(block, chunks[0]);

        let paragraph = Paragraph::new("Press 'q' to quit.").block(Block::default().borders(Borders::ALL));
        frame.render_widget(paragraph, chunks[1]);
    }

    fn main (5) {
        let x = 12;
        println!{"{}", x};

        let mut x = 12;
        println!{"before:{}", x};

        let x = 1:
        println!{"after:{}", x};
    }

fn main (6) { 
    /* only one winner out of 12 compwtitors and companies shall
    come out on top and be give the kengan association
    head chairman with a total of seven thrillion us dollars 
     */
    let winner = "OHMA TOKITA";
    println!("The winner of this tournament is:{}", winner);
    
    let company winner = "GENERATOR AND MOTOR REPUBLIC";
    println!("The company that wins the kengan death or annilation
    tournament is the one and only:{}", company winner);

    let second position = "FANG OF METSUDO AND DANIPPON BANK";
    println!("The 1st runner up to ohma tokita: {}", second position);

    let third position = "THE ASHURA";
    println!("The second runner up is: {}", third position)

    /* BOMBS ADDED TO THE DEATH ARENA AND IS BEING SNIFFED OUT
    BY A GROUP OF OUTSIDERS WHO SWEARED ON THEIR LIFE TOO DO WHAT EVER
    THEIR MASTER WANT FROM THEM SO HE CAN BE HAPPY AND THE KURE
    FAMILY HAVING SAID TO BE AN EVOLUTION OF HUMANS IN COMBAT
     AND SAID TO HAVE DEMONS INSIDE THEM AND HAVE A LOT OF THINGS THAT MAKE THEM TO BE COME A  VERY GOOD PART OF THE FOLLOWING
     */
}
