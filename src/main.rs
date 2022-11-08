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

    println!("The Twelve Days of Christmas");
    println!("");

    for ordinal in ordinal_numbers.into_iter() {
        println!("On the {} day of Christmas", ordinal);
        
        match ordinal {
            "First" | "Second" | "Third" | "Fourth" | "Fifth" | "Sixth" | "Seventh" | "Eighth" | "Ninth" | "Tenth" | "Eleventh" | "Twelfth" => {
                println!("My good friends brought to me");

                if ordinal.eq("Twelfth") {
                    println!("All their good wishes");
                }

                if ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Gifts for one and all");
                }

                if ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Some mistletoe");
                }

                if ordinal.eq("Ninth") || ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("A guardian angel");
                }

                if ordinal.eq("Eighth") || ordinal.eq("Ninth") || ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Gold and silver tinsel");
                }

                if ordinal.eq("Seventh") || ordinal.eq("Eighth") || ordinal.eq("Ninth") ||ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Candles a-glowing");
                }

                if ordinal.eq("Sixth") || ordinal.eq("Seventh") || ordinal.eq("Eighth") || ordinal.eq("Ninth") || ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Little silver bells");
                }

                if ordinal.eq("Fifth") || ordinal.eq("Sixth") || ordinal.eq("Seventh") || ordinal.eq("Eighth") || ordinal.eq("Ninth") || ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("A shining star");
                }

                if ordinal.eq("Fourth") || ordinal.eq("Fifth") || ordinal.eq("Sixth") || ordinal.eq("Seventh") || ordinal.eq("Eighth") || ordinal.eq("Ninth") || ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Four colored lights");
                }

                if ordinal.eq("Third") || ordinal.eq("Fourth") || ordinal.eq("Fifth") || ordinal.eq("Sixth") || ordinal.eq("Seventh") || ordinal.eq("Eighth") || ordinal.eq("Ninth") ||ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Three boughs of holly");
                }

                if ordinal.eq("Second") || ordinal.eq("Third") || ordinal.eq("Fourth") || ordinal.eq("Fifth") || ordinal.eq("Sixth") || ordinal.eq("Seventh") || ordinal.eq("Eighth") || ordinal.eq("Ninth") ||ordinal.eq("Tenth") || ordinal.eq("Eleventh") || ordinal.eq("Twelfth") {
                    println!("Two candy canes");
                }

                println!("And a song for the Christmas tree");
                println!(""); 
            },
            _ => { 
                println!(""); 
            },
        }
    }
}
