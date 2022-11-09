fn main() {

    let ordinal_numbers: [&str; 12]  = [ 
        "First", 
        "Second", 
        "Third", 
        "Fourth", 
        "Fifth", 
        "Sixth", 
        "Seventh",
        "Eighth", 
        "Ninth", 
        "Tenth", 
        "Eleventh", 
        "Twelfth"
    ];

    let phrases: [&str; 12]  = [
        "My good friends brought to me",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes"
    ];

    println!("The Twelve Days of Christmas");
    println!("");

    let mut i: i32 = 0;

    for ordinal in ordinal_numbers.into_iter() {
        
        println!("On the {} day of Christmas", ordinal);

        i = i + 1;

        for _n in 0..i {
            println!("{}", phrases[_n as usize]);
        }


        println!("And a song for the Christmas tree");
        println!(""); 
    }
}
