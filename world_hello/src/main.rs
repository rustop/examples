fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    let _ = String::from("Hello, world!");

    for region in regions.iter() {
        println!("{}", &region);
    }

    for region in regions {
        println!("{}", region);
    }


}

fn main() {
    greet_world();
}
