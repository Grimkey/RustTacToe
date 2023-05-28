pub mod term {
    // Does not move the cursor back to (1,1)
    //static clearScreenOnly: String = format!("{}[2J", 27 as char);

    // Clear the screen and move to position (1,1)
    pub fn clear_screen() -> String { format!("{esc}[2J{esc}[1;1H", esc = 27 as char) }
}