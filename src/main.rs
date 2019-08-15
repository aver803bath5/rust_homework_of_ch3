fn main() {
    let mut fahreheit = 100.0;
    let mut celsius = fahrenheit_to_celsius(fahreheit);
    println!("{} F = {} C", fahreheit, celsius);

    celsius = 100.0;
    fahreheit = celsius_to_fahrenheit(100.0);
    println!("{} C = {} F", celsius, fahreheit);

    for i in 0..10 {
        println!("{}", fabonacci(i));
    }

    the_twelve_days_of_chrismas();
}

// Turn Fahrenheit into Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Turn Celsius into Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// Calculate nth Fibonacci number
fn fabonacci(n: u64) -> u64 {
    let mut x: u64 = 0;
    let mut y: u64 = 1;
    let mut z: u64 = 1;
    let mut temp: u64;

    if n == 0 {
        n
    } else {
        for _ in 1..n {
            temp = x + y;
            x = y;
            z = temp;
            y = z;
        }
        z
    }
}

// The lyrics of "The Twelve Days of Chrismas"
fn the_twelve_days_of_chrismas() {
    let nth_day = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let new_lines = [
        "A partridge in a pear tree.\n",
        "Two turtle doves,\nAnd a partridge in a pear tree.\n",
        "Three French hens,\n",
        "Four calling birds,\n",
        "Five golden rings,\n",
        "Six geese a-laying,\n",
        "Seven swans a-swimming,\n",
        "Eight maids a-milking,\n",
        "Nine ladies dancing,\n",
        "Ten lords a-leaping,\n",
        "Eleven pipers piping,\n",
        "Twelve drummers drumming,",
    ];

    let mut lyrics = String::new();
    for i in 0..12 {
        println!("On the {} day of Chrismas,", nth_day[i]);
        println!("My true love send me");
        if i == 0 {
            println!("{}", new_lines[i]);
        } else {
            // Insert lyrics from the begigging of the lyrics string
            lyrics.insert_str(0, new_lines[i]);
            println!("{}", lyrics);
        }
    }
}
