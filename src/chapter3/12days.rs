fn main(){
    let stringindex : [(&str, &str); 12] = [("First", "A partridge in a pear tree"), 
                                                ("Second", "Two Turtle Doves"),
                                                ("Third", "Three French Hens"),
                                                ("Fourth", "Four Calling Birds"),
                                                ("Fifth", "Five Golden Rings"),
                                                ("Sixth", "Six Geese a Laying"),
                                                ("Seventh", "Seven Swans a Swimming"),
                                                ("Eighth", "Maids a Milking"),
                                                ("Ninth", "Nine Ladies Dancing"),
                                                ("Tenth", "Ten Lords a Leaping"),
                                                ("Eleventh", "Eleven Pipers Piping"),
                                                ("Twelfth", "Twelve Drummers Drumming")];
    let mut stringbuffer = String::new();

    for x in 0..stringindex.len(){
        stringbuffer.push_str("On the ");
        stringbuffer.push_str(stringindex[x].0);
        stringbuffer.push_str(" day of Christmas my true love gave to me: ");
        for x2 in (0..x).rev(){
            stringbuffer.push_str(stringindex[x2].1);
            stringbuffer.push_str(", ");
        }
        println!("{ }", stringbuffer);
        stringbuffer.clear();
    }
}
