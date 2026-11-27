pub const SIGNATURE: &str = "— Le Père Noël";

pub const LINES: [&str; 5] = [
    "Merci d'avoir pris soin de Chocolutin tout le mois de décembre.",
    "J'espère qu'il n'a pas fait n'importe quoi… Je le connais, je préfère ne pas trop demander.",
    "Pour te remercier, je t'ai laissé un bébé renne au jardin.",
    "Il est encore petit, mais il a bon cœur.",
    "Joyeux Noël\u{202F}!",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_santa_note_thanks_for_chocolutin_and_gives_the_reindeer() {
        assert!(LINES[0].contains("Chocolutin"));
        assert!(LINES.iter().any(|line| line.contains("n'importe quoi")));
        assert!(LINES.iter().any(|line| line.contains("bébé renne")));
    }

    #[test]
    fn test_santa_note_keeps_french_spacing_and_no_elf_tic() {
        for line in LINES.iter().chain([&SIGNATURE]) {
            assert!(!line.to_lowercase().contains("hop"), "{line}");
            for mark in ['!', '?', ':', ';'] {
                assert!(!line.contains(&format!(" {mark}")), "{line}");
            }
        }
    }
}
