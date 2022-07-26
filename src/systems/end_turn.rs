use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Score)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map, #[resource] final_score: &mut i32, #[resource] score_message: &mut String) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet
        .iter(ecs)
        .nth(0)
        .unwrap_or(&amulet_default);

    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return, //nothing to do
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };    

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            println!("end_turn.rs changing to game over");
            thread::spawn(|| {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();
                let file = BufReader::new(File::open("../resources/Player_Death_Scream.wav").unwrap());
                let source = Decoder::new(file).unwrap();
                sink.append(source);
                sink.sleep_until_end();
            });
            score_handle(&ecs.clone(), final_score, score_message);
            new_state = TurnState::GameOver;
        }
        if pos == amulet_pos {
            thread::spawn(|| {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();
                let file = BufReader::new(File::open("../resources/Victory.wav").unwrap());
                let source = Decoder::new(file).unwrap();
                sink.append(source);
                sink.sleep_until_end();
            });
            score_handle(&ecs.clone(), final_score, score_message);
            new_state = TurnState::Victory;
        }

        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });

    *turn_state = new_state;
}

fn score_handle(ecs: &SubWorld, final_score: &mut i32, score_message: &mut String) {
    match Path::new(SCORES_LOC).exists() {
        true => {
            let mut player_score_q = <&Score>::query().filter(component::<Player>());
            let player_score = player_score_q
                .iter(ecs)
                .nth(0)
                .unwrap();

            let scores_file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(SCORES_LOC);
            match scores_file {
                Ok(mut file) => {
                    let mut file_contents = String::new();
                    file.read_to_string(&mut file_contents).unwrap();
                    let results_vec: Vec<&str> = file_contents.split('\n').collect();
                    let mut scores_vec: Vec<i32> = Vec::new();
                    for result in results_vec.iter() {
                        let temp: &str = &result.chars().filter(|c| c.is_digit(10)).collect::<String>();
                        match temp.parse::<i32>() {
                            Ok(num) => scores_vec.push(num),
                            _ => {}
                        }
                    }   //for-loop building a list of the scores from the file
                    match vec_max(&scores_vec) {
                        Some(max) => {
                            let max_score = max;
                            let new_score = player_score.current;
                            let mut score_string = String::new();
                            if !scores_vec.contains(&new_score) {
                                scores_vec.push(new_score);
                            }                       //ensure that duplicate scores are not logged
                            scores_vec.sort();      //sort the vec in ascending order (worst-case O(n * log(n)))
                            scores_vec.reverse();   //reverse the vec so that high score is at top when written to file

                            if new_score > max_score {
                                *score_message = String::from("New High Score!!!");
                            } else {
                                *score_message = String::new();
                            }   //notify of high score or not

                            *final_score = new_score;
                            println!("final score set to : {}", final_score);
                            for score in scores_vec.iter() {
                                let temp = score.to_string() + "\n";
                                score_string.push_str(temp.as_str());
                            }

                            match File::create(SCORES_LOC) {
                                Ok(mut new_file) => {
                                    match write!(new_file, "{}", score_string) {
                                        Ok(_result) => {},
                                        _ => {}
                                    }
                                },
                                Err(e) => panic!("Problem creating the file {:?}", e)
                            }   //wipe contents of the file and write to it the new scores list
                        },
                        None => {}
                    }
                    
                },
                Err(_e) => {}
            }
        },
        false => {}
    }
}

fn vec_max(vector: &Vec<i32>) -> Option<i32> {
    if vector.len() != 0 {
        let mut temp_max = vector[0];
        for val in vector.iter() {
            if *val >= temp_max {
                temp_max = *val;
            }
        }
        Some(temp_max)
    } else {
        Some(0)
    }
}