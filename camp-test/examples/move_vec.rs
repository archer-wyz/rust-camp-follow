fn main() {
    for_loop_example();
    vec_take()
}

fn for_loop_example() {
    let composers = vec![
        "Palestrina".to_string(),
        "Dowland".to_string(),
        "Lully".to_string(),
        "Purcell".to_string(),
    ];
    // composers implicitly use into_iter() method, leading move the ownership of composers to the
    // for loop; so composers is no longer available after the loop.
    for mut composer in composers {
        composer.push('!');
    }
}

fn vec_take() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        birth: i32,
        first_name: String,
    }

    let mut persons = vec![
        Person {
            name: Some("Palestrina".to_string()),
            birth: 1525,
            first_name: "Giovanni".to_string(),
        },
        Person {
            name: Some("Dowland".to_string()),
            birth: 1563,
            first_name: "John".to_string(),
        },
        Person {
            name: Some("Lully".to_string()),
            birth: 1632,
            first_name: "Jean-Baptiste".to_string(),
        },
        Person {
            name: Some("Purcell".to_string()),
            birth: 1659,
            first_name: "Henry".to_string(),
        },
    ];
    // Option 替换为 None -> take() 新建了一个 Option<String> 并将 persons[0].name
    // 值的所有权转移给了新建的 Option<String>, 之后 persons[0].name 为 None
    // 整个过程中，persons[0].name 本身的所有权并没有转移
    let name = persons[0].name.take();
    // 基本类型，move 自动转变成 Copy
    let birth = persons[1].birth;
    // 尝试从 Vec 中 move 出一个元素，是不允许的
    //let first_name = persons[2].first_name;
    let first_name = persons[2].first_name.clone();
    persons[3].first_name.push('1');

    {
        let person_3 = &mut persons[3];
        person_3.first_name.push('2');
        let person_3_2 = persons.get_mut(3).unwrap();
        person_3_2.first_name.push('3');
        // &mut 只能存在一个 --> 可变引用只能被 borrow 一次
        // person_3.birth = 3;
    }

    println!("{:?}", name);
    println!("{:?}", birth);
    println!("{:?}", first_name);
    println!("{:?}", persons);
}
