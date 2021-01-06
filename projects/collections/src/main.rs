use std::collections::HashMap;

fn main() {
    println!("Hello, collections!");
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    println!("v: {:#?}", v);
    let v_int = vec![1, 2, 3];
    let mut v_uint: Vec<u8> = vec![4, 5, 6];
    let nine = String::from("nine");
    let mut v_str = Vec::from(["seven".to_string(), "eight".to_string(), nine]);
    let nine = &v_str[2];
    println!("nine: {}", nine);
    let bad = match v_str.get(3) {
        Some(fourth) => fourth,
        None => "There is no fourth element.",
    };
    println!("bad: {:#?}", bad);
    // Can't call this here because we're using the
    // "nine" reference in the println below:
    // v_str.push("appended".to_string());
    println!(
        "v_int: {:#?}, v_uint: {:#?}, v_str: {:#?}, nine: {}",
        v_int, v_uint, v_str, nine
    );
    // It's fine here though
    v_str.push("appended".to_string());
    println!("{:#?}", v_str);

    for i in &mut v_uint {
        *i += 200;
    }
    println!("v_uint after mutation {:#?}", v_uint);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("enum vector: {:#?}", row);

    let mut s = String::new();
    s += "MOAR TEXT";
    println!("s: >>{}<<", s);

    let mut hellos = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    hellos[4].push_str("_PUSHED");
    hellos[4].push('!');
    for hello in hellos.iter() {
        println!("{}", hello);
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s2_slice = " Glad you're still here!";
    let s3 = s1 + &s2 + s2_slice; // note s1 has been moved here and can no longer be used
    println!("s3: {}", s3);
    let s1 = "tic";
    let s2 = "tac";
    let s3 = "toe";
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("The best game ever: {}", s);
    println!("TOE: {}", s3);
    let e_slice = &s3[2..3];
    let mut e_char = '_';
    for (i, c) in s3.chars().enumerate() {
        if i == 2 {
            e_char = c;
            break;
        }
    }
    let e_char_byte = s3.as_bytes()[2] as char;
    println!("e should be {}, {}, {}", e_slice, e_char, e_char_byte);

    // let old_scores = {
    //     "Green": 90
    // };

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // let mut scores = HashMap::new();

    scores.insert(String::from("Purple"), 10);
    scores.insert(String::from("Orange"), 50);
    println!("scores: {:#?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("field_name: {}", map["Favorite color"]);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score: {:#?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    {
        let chartreuse_team = scores.entry(String::from("Chartreuse")).or_insert(75);
        *chartreuse_team += 10;
    }
    let blue_team = scores.entry(String::from("Blue")).or_insert(-99);
    *blue_team += 1;
    let yellow_team = scores.entry(String::from("Yellow")).or_insert(-999);
    *yellow_team += 1;
    // Can't print both entries due to scores ownership!
    // println!("{}, {}", blue_team, yellow_team);

    println!("{:?}", scores);

    let text = "happy happy joy joy is what happy people say";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let v_integers = vec![2001, 1999, 2020, 1950, 1975, 2021, 2021];
    println!("median of {:#?} is {}", v_integers, median(&v_integers));
    println!(
        "mean of {:#?} is {}, from a sum: {}, mode is {}",
        v_integers,
        mean(&v_integers),
        sum(&v_integers),
        mode(&v_integers)
    );
    let pl_input = "hello friends! Fortuitously, I am happy to meet you!";
    println!(
        "\"{}\" in pig latin is \"{}\"",
        pl_input,
        pig_latin(&pl_input)
    );
    println!(
        "\"{}\" in pig latin compact is \"{}\"",
        pl_input,
        pig_latin_compact(&pl_input)
    );}

fn median(v_integers: &Vec<i32>) -> i32 {
    // let middle = v_integers.len() / 2 as usize;
    // TODO(gavin): cool 1-liner?
    // let med = v_integers.as_mut<Vec<i32>>().sort_unstable()[v_integers.len() / 2 as usize];
    // let med = v_integers.clone().sort_unstable()[(v_integers.len() / 2 as usize)];
    let mut sorted = v_integers.clone();
    sorted.sort();
    let mid: usize = sorted.len() / 2;
    return sorted[mid];
}

fn sum(v_integers: &Vec<i32>) -> i32 {
    let mut a = 0;
    for i in v_integers {
        a += i;
    }
    a
}

fn mean(v_integers: &Vec<i32>) -> i32 {
    return sum(v_integers) / v_integers.len() as i32;
}

// Returns the number that appears most often or
// the first number in the vector if they all appear
// an equal number of times.
fn mode(v_integers: &Vec<i32>) -> i32 {
    let mut mode: &i32 = &-1;
    let mut count_map = HashMap::new();
    let mut current_high_count: i32 = 0;
    for i in v_integers {
        let updated = count_map.entry(i).or_insert(0);
        *updated += 1;
        if *updated > current_high_count {
            mode = i;
            current_high_count = *updated;
        }
    }
    return *mode;
}

fn pigify(word: &str, vowels: &[bool; 256]) -> String {
    if word.len() < 1 {
        return word.to_string();
    }
    let mut out = String::with_capacity(word.len() * 2);
    let word_bytes = word.as_bytes();
    let first_letter = word_bytes[0] as char;
    if is_char(first_letter, vowels) {
        out.push_str(&word);
        out.push_str("-hay");
    } else {
        let bytes: &[u8] = &word_bytes[1..];
        let v_bytes: Vec<u8> = Vec::from(bytes);
        let word = match String::from_utf8(v_bytes) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        out.push_str(&word);
        out.push('-');
        out.push(first_letter);
        out.push_str("ay");
    }
    return out;
}

fn pig_latin_compact(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    let vowels = init_char_map(&VOWELS);
    for word in input.split(&BREAKS[..]) {
        output.push_str(&pigify(word, &vowels));
        output.push(' ');
    }
    return output;
}

// Convert strings to pig latin. The first consonant of each word is moved to the end
// of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
// a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in
// mind the details about UTF-8 encoding!
fn pig_latin(input: &str) -> String {
    // It's a good bet that the output will never be more
    // than 2x the input, so we can save an allocation in
    // most cases:
    let mut output = String::with_capacity(input.len() * 2);
    // We keep re-using this word but 128 is larger than
    // most words are likely to be:
    let mut word = String::with_capacity(128);
    let mut new_word = true;
    let mut suffix = '_';
    let vowels = init_char_map(&VOWELS);
    let breaks = init_char_map(&BREAKS);

    for c in input.chars() {
        if is_char(c, &breaks) {
            if new_word {
                output.push(c);
            } else {
                add_pig(&mut output, &word, suffix, c);
                word.clear();
                new_word = true;
            }
            continue;
        }
        if new_word {
            if is_char(c, &vowels) {
                suffix = 'h';
                word.push(c);
            } else {
                suffix = c;
            }
            new_word = false;
        } else {
            word.push(c);
        }
    }
    output.push_str(&word);
    return output;
}

fn add_pig(output: &mut String, word: &str, suffix: char, c: char) {
    output.push_str(word);
    output.push('-');
    output.push(suffix);
    output.push_str("ay");
    output.push(c);
}

const VOWELS: [char; 12] = ['a', 'e', 'i', 'o', 'u', 'y', 'A', 'E', 'I', 'O', 'U', 'Y'];
const BREAKS: [char; 3] = [' ', '!', ','];

fn init_char_map(chars: &[char]) -> [bool; 256] {
    let mut v_map: [bool; 256] = [false; 256];
    for v in chars.iter() {
        v_map[*v as usize] = true;
    }
    return v_map;
}

fn is_char(c: char, char_map: &[bool; 256]) -> bool {
    return char_map[c as usize];
}

// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department
// in a company. For example, “Add Sally to Engineering”
// or “Add Amir to Sales.” Then let the user retrieve a
// list of all people in a department or all people in
// the company by department, sorted alphabetically.
fn hr_update() {
    // let eng = Department::from("Engineering");
    // let sales = Department::from("Sales");

    // let mut departments = HashMap::<Department, HashMap<String, Person>>::new();
    // let mut eng_people = HashMap::new();
    // eng_people.insert("Sally".to_string(), sally);
    // departments.insert(eng, sally);

    // let mut &eng_people = departments.entry(eng).or_insert(HashMap::new());
    //let mut eng_people = departments.entry(eng).or_insert(HashMap::new());

    //let mut &people = departments.entry(eng).or_insert(Vec::new());
}

#[derive(Debug)]
struct Company<'a> {
    name: String,
    departments: HashMap<String, Department<'a>>,
    employees: HashMap<String, Person>
}

impl Company<'_> {
    fn from(name: &str) -> Company {
        return Company{name: name.to_string(), departments: HashMap::new(), employees: HashMap::new()};
    }
    fn add_employee(&self, name: &str, department: &str) -> (&Department, &Person) {
        let department = self.departments.entry(department.to_string()).or_insert(Department::from(department));
        let person = self.employees.entry(name.to_string()).or_insert(Person{name: name.to_string()});
        department.add_person(person);
        return (department, person);
    }
}

#[derive(Debug)]
struct Department<'a> {
    name: String,
    people: HashMap<String, &'a Person>
}

impl<'a> Department<'a> {
    fn from(name: &str) -> Department {
        return Department{name: name.to_string(), people: HashMap::new()}
    }

    fn add_person(&self, person: &'a Person) {
        return self.people.insert(person.name, person).expect("Error adding person");
    }
}


#[derive(Debug)]
struct Person {
    name: String
}
