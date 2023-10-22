fn main() {
    {
        // Vec<T>
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        for val in v {
            println!("val: {}", val);
        }
    } 

    {
        let _v2 = vec![1, 2, 3, 4, 5];
        // v2 valid inside here
    } // <- v2 is dropped here
    
    {
        let v3 = vec![1, 2, 3, 4, 5];
        let third = &v3[2];
        println!("third: {:?}", third);
        
        match v3.get(2) {
            Some(val) => println!("element found: {}", val),
            None => println!("element not found")
        }
    }

    {
        let v4 = vec![1, 2, 3, 4, 5];
        // let not_found_index = &v4[50];
        let not_found_index = &v4.get(50);
        println!("{:?}", not_found_index);

    }

    {
        let mut v5 = vec![1, 2, 3, 4, 5];
        let first = &v5[0];
        println!("{}", first);

        v5.push(6);
    }

    {
        let v6 = vec![1, 2, 3];
        for i in &v6 {
            println!("{}", i);
        }
    }
    
    {
        let mut v7 = vec![1, 2, 3];
        for i in &mut v7 {
            *i += 10;
        }
        println!("v7 = {:?}", v7);
    }
    {
        #[derive(Debug)]
        enum Excel {
            Int(i32),
            Float(f64),
            Text(String)
        }

        let row = vec![
            Excel::Int(5),
            Excel::Text(String::from("Hello World")),
            Excel::Float(3.14),
        ];

        if let Some(val) = row.get(1) {
            if let Excel::Text(kotoba) = val {
                println!("row baris 2: {}", kotoba);
            }
        }

        match row.get(1) {
            Some(Excel::Text(kotoba)) => {
                println!("{:?}", kotoba);
            }
            _ => println!("Bukan Excel::String")
        }

        match row.get(1) {
            Some(is_excel) => match is_excel {
                Excel::Text(kotoba) => println!("kotoba: {}", kotoba),
                _ => println!("bukan excel")
            }
            None => println!("diluar nurul")
        }
    }
}
