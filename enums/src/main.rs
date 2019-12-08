#[derive(Debug)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

#[derive(Debug)]
struct DiaryEntry {
    entry_day: DayOfWeek,
    entry: String
}

#[derive(Debug)]
struct Ipv4Addr {
    a1: u8,
    a2: u8,
    a3: u8,
    a4: u8
}

#[derive(Debug)]
struct Ipv6Addr {
    a1: u8,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

fn main() {

    // Create some diary entries
    let de1 = DiaryEntry { entry_day: DayOfWeek::Monday, entry: String::from("Went shopping with the kids") };
    let de2 = DiaryEntry { entry_day: DayOfWeek::Tuesday, entry: String::from("Made some toast") };
    let de3 = DiaryEntry { entry_day: DayOfWeek::Wednesday, entry: String::from("Went sky diving") };
    let de4 = DiaryEntry { entry_day: DayOfWeek::Sunday, entry: String::from("Flew a kite with the kids") };

    // Store them in a diary (vector)
    let diary = vec![de1, de2, de3, de4];

    // Print out all diary entries
    for diary_entry in diary.iter() {
        println!("Entry for {:?} is a {:?}", diary_entry.entry_day, diary_entry.entry);
    }


    // Create some IP address in V4 and V6 format
    let ip1 = IpAddr::V4(Ipv4Addr { a1: 192, a2: 168, a3: 15, a4: 2 });
    let ip2 = IpAddr::V6(Ipv6Addr { a1: 1, a2: 2, a3: 3, a4: 4, a5: 5, a6: 6 });

    // Print the address out
    println!("ip1 = {:?}", ip1);    
    println!("ip2 = {:?}", ip2);    
}
