struct Data {
    field1: String,
    field2: String,
    field3: u64,
    field4: i64,
}

fn main() {
    let data1 = build_data (
        String::from("This is the dats for field1"),
        String::from("Dst for fied2"),
        45,
    );

    let data2 = Data {
        field1: String::from("Text for data2.field1"),
        field2: String::from("Text for data2.field2"),
        ..data1         // Shorthand to init the rest of the structure with the same information from data1 struct
    };

    println!("data1 = field1:[{}], field2:[{}], field3:[{}], field4:[{}]", data1.field1, data1.field2, data1.field3, data1.field4);
    println!("data2 = field1:[{}], field2:[{}], field3:[{}], field4:[{}]", data2.field1, data2.field2, data2.field3, data2.field4);

    // Create a Tuple Struct (named tuple) and print out its values
    let named_tuple = build_named_tuple(1,2,3);
    println!("named_tuple=({},{},{})", named_tuple.0, named_tuple.1, named_tuple.2);
}

// A function that builds the structure from passed in values. Rather than initilizing the
// structs fields manually the shorthand mechanism is used where the fn parameters and the structs field
// names match. The last field of the struct is manually initialized,
fn build_data(field1: String, field2: String, field3: u64) -> Data {
    Data {
        field1,
        field2,
        field3,
        field4: -4567,
    }
}

struct NamedTuple (i64, i64, i64);

// Create a named tuple function
fn build_named_tuple(one: i64, two: i64, three: i64) -> NamedTuple {
    NamedTuple(one, two, three)
}