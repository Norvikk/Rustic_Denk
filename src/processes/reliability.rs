use crate::actions;
use crate::processes;
use crate::utility;
use crate::ProcessConfig;

use colored;
use colored::Colorize;
use std::collections::HashMap;

pub fn reliability_process(loops: usize, to_display: bool) {
    for index in (5..=loops).step_by(5) {
        let lower_letter_to_assert =
            utility::generate::custom_random_string(true, false, false, false, 25);
        let number_to_assert =
            utility::generate::custom_random_string(false, true, false, false, 15);
        let symbol_to_assert =
            utility::generate::custom_random_string(false, false, true, false, 25);
        let upper_letter_to_assert =
            utility::generate::custom_random_string(false, false, false, true, 25);
        let all_characters_to_assert =
            utility::generate::custom_random_string(true, true, true, true, 25);

        let mut config_numbers: ProcessConfig = ProcessConfig {
            user_key_length: utility::generate::custom_random_string(false, true, false, false, 4)
                .parse::<usize>()
                .unwrap(),
            user_clear_payload: number_to_assert.clone(),
            system_synapse: utility::generate::random_string(10),
            process_soft_bundle: HashMap::new(),
            process_created_blur: String::new(),
            read_blur: String::new(),
            read_keys: String::new(),
        };

        let number_result = return_clear_payload(&mut config_numbers);
        if to_display {
            println!(
                "User Clear Payload: {}\t      | User Key Length: {}",
                config_numbers.user_clear_payload.on_bright_white().black(),
                config_numbers.user_key_length
            );
        }
        assert_eq!(number_to_assert, number_result);
        config_numbers.flush();

        let mut config_lower_letter: ProcessConfig = ProcessConfig {
            user_key_length: utility::generate::custom_random_string(false, true, false, false, 4)
                .parse::<usize>()
                .unwrap(),
            user_clear_payload: lower_letter_to_assert.clone(),
            system_synapse: utility::generate::random_string(10),
            process_soft_bundle: HashMap::new(),
            process_created_blur: String::new(),
            read_blur: String::new(),
            read_keys: String::new(),
        };

        let lower_letter_result = return_clear_payload(&mut config_lower_letter);
        if to_display {
            println!(
                "User Clear Payload: {} | User Key Length: {}",
                config_lower_letter
                    .user_clear_payload
                    .on_bright_white()
                    .black(),
                config_lower_letter.user_key_length
            );
        }
        assert_eq!(lower_letter_to_assert, lower_letter_result);
        config_lower_letter.flush();

        let mut config_symbols: ProcessConfig = ProcessConfig {
            user_key_length: utility::generate::custom_random_string(false, true, false, false, 4)
                .parse::<usize>()
                .unwrap(),
            user_clear_payload: symbol_to_assert.clone(),
            system_synapse: utility::generate::random_string(10),
            process_soft_bundle: HashMap::new(),
            process_created_blur: String::new(),
            read_blur: String::new(),
            read_keys: String::new(),
        };

        let symbol_result = return_clear_payload(&mut config_symbols);
        if to_display {
            println!(
                "User Clear Payload: {} | User Key Length: {}",
                config_symbols.user_clear_payload.on_bright_white().black(),
                config_symbols.user_key_length
            );
        }
        assert_eq!(symbol_to_assert, symbol_result);
        config_symbols.flush();

        let mut config_upper_letters: ProcessConfig = ProcessConfig {
            user_key_length: utility::generate::custom_random_string(false, true, false, false, 4)
                .parse::<usize>()
                .unwrap(),
            user_clear_payload: upper_letter_to_assert.clone(),
            system_synapse: utility::generate::random_string(10),
            process_soft_bundle: HashMap::new(),
            process_created_blur: String::new(),
            read_blur: String::new(),
            read_keys: String::new(),
        };
        let upper_letter_result = return_clear_payload(&mut config_upper_letters);
        if to_display {
            println!(
                "User Clear Payload: {} | User Key Length: {}",
                config_upper_letters
                    .user_clear_payload
                    .on_bright_white()
                    .black(),
                config_upper_letters.user_key_length
            );
        }
        assert_eq!(upper_letter_to_assert, upper_letter_result);
        config_upper_letters.flush();

        let mut config_all_characters: ProcessConfig = ProcessConfig {
            user_key_length: utility::generate::custom_random_string(false, true, false, false, 4)
                .parse::<usize>()
                .unwrap(),
            user_clear_payload: all_characters_to_assert.clone(),
            system_synapse: utility::generate::random_string(10),
            process_soft_bundle: HashMap::new(),
            process_created_blur: String::new(),
            read_blur: String::new(),
            read_keys: String::new(),
        };

        let all_characters_result = return_clear_payload(&mut config_all_characters);
        if to_display {
            println!(
                "User Clear Payload: {} | User Key Length: {}",
                config_all_characters
                    .user_clear_payload
                    .on_bright_white()
                    .black(),
                config_all_characters.user_key_length
            );
        }
        assert_eq!(all_characters_to_assert, all_characters_result);
        config_all_characters.flush();

        if to_display {
        println!(
            "\n{}{}",
            index.to_string().green().bold(),
            " iterations finished".green().bold()
        );
    }
    }
    fn return_clear_payload(config: &mut ProcessConfig) -> String {
        processes::encrypt::encrypt(config);
        actions::write::files(config, ".dnk");
        config.flush_user_clear_payload();
        processes::decrypt::decrypt(config);
        config.user_clear_payload.clone()
    }
}
