use std::io;
use std::io::Write;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    /* let words = HashMap::from([
        ("tulla joksikin", ["bli", "blir", "blev", "blivit"]),
        ("juoda", ["dricka", "dricker", "drack", "druckit"]),
        ("olla olemassa", ["finnas", "finns", "fanns", "funnits"]),
        ("lentää", ["flyga", "flyger", "flög", "flugit"]),
        ("saada", ["få", "får", "fick", "fått"]),
        ("ymmärtää", ["förstå", "förstår", "förstod", "förstått"]),
        ("mennä, kävellä", ["gå", "går", "gick", "gått"]),
        ("tehdä", ["göra", "gör", "gjorde", "gjort"]),
        ("olla, omistaa", ["ha", "har", "hade", "haft"]),
        ("olla nimeltään", ["heta", "heter", "hette", "hetat"]),
    ]); */
    /* let words = HashMap::from([
        ("hakata", ["hugga", "hugger", "högg", "huggit"]),
        ("pitää", ["hålla", "håller", "höll", "hållit"]),
        ("tulla", ["komma", "kommer", "kom", "kommit"]),
        ("osata", ["kunna", "kan", "kunde", "kunnat"]),
        ("maata, sijaita", ["ligga", "ligger", "låg", "legat"]),
        ("kuulostaa", ["låta", "låter", "lät", "låtit"]),
        ("laittaa", ["lägga", "lägger", "lade", "lagt"]),
        ("nähdä", ["se", "ser", "såg", "sett"]),
        ("istua", ["sitta", "sitter", "satt", "suttit"]),
        ("laulaa", ["sjunga", "sjunger", "sjöng", "sjungit"]),
    ]); */
    let words = HashMap::from([
        ("tulla joksikin", ["bli", "blir", "blev", "blivit"]),
        ("juoda", ["dricka", "dricker", "drack", "druckit"]),
        ("olla olemassa", ["finnas", "finns", "fanns", "funnits"]),
        ("lentää", ["flyga", "flyger", "flög", "flugit"]),
        ("saada", ["få", "får", "fick", "fått"]),
        ("ymmärtää", ["förstå", "förstår", "förstod", "förstått"]),
        ("mennä, kävellä", ["gå", "går", "gick", "gått"]),
        ("tehdä", ["göra", "gör", "gjorde", "gjort"]),
        ("olla, omistaa", ["ha", "har", "hade", "haft"]),
        ("olla nimeltään", ["heta", "heter", "hette", "hetat"]),
        ("hakata", ["hugga", "hugger", "högg", "huggit"]),
        ("pitää", ["hålla", "håller", "höll", "hållit"]),
        ("tulla", ["komma", "kommer", "kom", "kommit"]),
        ("osata", ["kunna", "kan", "kunde", "kunnat"]),
        ("maata, sijaita", ["ligga", "ligger", "låg", "legat"]),
        ("kuulostaa", ["låta", "låter", "lät", "låtit"]),
        ("laittaa", ["lägga", "lägger", "lade", "lagt"]),
        ("nähdä", ["se", "ser", "såg", "sett"]),
        ("istua", ["sitta", "sitter", "satt", "suttit"]),
        ("laulaa", ["sjunga", "sjunger", "sjöng", "sjungit"]),
    ]);


    println!("Anna epäsäännöllisen verbin taivutus muodot ruotsiksi: esim. bli, blir, blev, blivit \n");

    let mut rng = rand::thread_rng();

    let mut keys: Vec<&str> = words.keys().cloned().collect();

    let mut failed_keys: Vec<&str> = vec![];

    'main: loop {
        for _i in 0..keys.len() {
            let key = keys.remove(rng.gen_range(0..keys.len()));
            'inner: loop { 
                let mut input_array: [String; 4] = ["".to_string(), "".to_string(), "".to_string(), "".to_string()];
                let input = get_input(format!("{}: ", &key));
                if input.split(',').count() == 1 {
                    println!("ohitettin...");
                    failed_keys.push(key);
                    break 'inner;
                }
                if input.split(',').count() != 4 {
                    println!("Yritä uudelleen (huono input)");
                    continue;
                }
                for (i, x) in input.split(',').enumerate() {
                    let trimmed: String = x.split_whitespace().collect();
                    input_array[i] = trimmed.to_owned();
                }
                match words.get(key) {
                    Some(word_list) => {
                        if word_list.eq(&input_array) {
                            println!("Oikein!");
                        } else {
                            println!("Väärin!");
                            failed_keys.push(key);
                        }
                    },
                    None => println!("Error")
                }
                break 'inner;
            }
        }
        if failed_keys.len() == 0 {
            println!("Kaikki oikein!");
            break 'main;
        } else {
            println!("Väärin menneet: {:?}", &failed_keys.join(", "));
            keys = failed_keys.clone();
            failed_keys = vec![];
        }
        
    } 
}

fn get_input(prompt: String) -> String{
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}
