use super::Engine;

#[test]
fn should_return_correct_number_of_neighbours() {
    let data = vec![
        (0, 0, 3), (1, 0, 5), (2, 0, 3),
        (0, 1, 5), (1, 1, 8), (2, 1, 5),
        (0, 2, 3), (1, 2, 5), (2, 2, 3),
    ];
    let e = Engine::new(3, 3);
    for (x, y, count) in data {
        println!("{},{}: {}", x, y, count);
        assert_eq!(e.neighbour_iter(x, y).count(), count);
    }
}
