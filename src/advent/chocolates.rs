pub const DRAWING_WIDTH: usize = 8;

pub struct Chocolate {
    pub name: &'static str,
    pub drawing: [&'static str; 3],
}

pub const CHOCOLATES: [Chocolate; 24] = [
    Chocolate {
        name: "Truffe au champagne",
        drawing: ["  .--.  ", " (::::) ", "  `--`  "],
    },
    Chocolate {
        name: "Praliné noisette du Piémont",
        drawing: ["  ____  ", " |oooo| ", " |____| "],
    },
    Chocolate {
        name: "Ganache framboise",
        drawing: [" .----. ", " | @@ | ", " '----' "],
    },
    Chocolate {
        name: "Rocher aux amandes",
        drawing: ["  .:'.  ", " :'.:'. ", "  ':.'  "],
    },
    Chocolate {
        name: "Mendiant aux fruits secs",
        drawing: ["  ____  ", " /o*.o\\ ", " \\____/ "],
    },
    Chocolate {
        name: "Palet d'or",
        drawing: ["  _**_  ", " (____) ", "  `--`  "],
    },
    Chocolate {
        name: "Caramel au beurre salé",
        drawing: [" +----+ ", " |~~~~| ", " +----+ "],
    },
    Chocolate {
        name: "Orangette confite",
        drawing: ["    //  ", "   //   ", "  //    "],
    },
    Chocolate {
        name: "Griotte à l'eau-de-vie",
        drawing: ["    /   ", "  _/_   ", " (___)  "],
    },
    Chocolate {
        name: "Pavé à la fleur de sel",
        drawing: ["  ____  ", " /___/| ", " |___|/ "],
    },
    Chocolate {
        name: "Marron glacé",
        drawing: ["  .^.   ", " (   )  ", "  `-`   "],
    },
    Chocolate {
        name: "Bouchée pistache",
        drawing: ["  ____  ", " /####\\ ", " \\####/ "],
    },
    Chocolate {
        name: "Cœur fondant passion",
        drawing: [" /\\ /\\  ", " \\   /  ", "  \\_/   "],
    },
    Chocolate {
        name: "Lingot gianduja",
        drawing: ["   __   ", "  /  \\  ", " /____\\ "],
    },
    Chocolate {
        name: "Nougatine croquante",
        drawing: ["  ____  ", " <____> ", "  ^^^^  "],
    },
    Chocolate {
        name: "Florentin aux amandes",
        drawing: [" ,-~~-, ", " (oO o) ", " `-~~-` "],
    },
    Chocolate {
        name: "Grains de café enrobés",
        drawing: ["  _  _  ", " (|)(|) ", "  `  `  "],
    },
    Chocolate {
        name: "Feuilleté praliné",
        drawing: [" ====== ", " ------ ", " ====== "],
    },
    Chocolate {
        name: "Coquille noisette",
        drawing: ["  _,_   ", " /|||\\  ", " \\___/  "],
    },
    Chocolate {
        name: "Bâton menthe poivrée",
        drawing: ["  ||||  ", "  ||||  ", "  ''''  "],
    },
    Chocolate {
        name: "Guimauve enrobée",
        drawing: ["  .--.  ", " |    | ", " '----' "],
    },
    Chocolate {
        name: "Étoile au miel",
        drawing: ["   /\\   ", " <    > ", "   \\/   "],
    },
    Chocolate {
        name: "Cloche à la cannelle",
        drawing: ["  /^^\\  ", " /    \\ ", " '-o--' "],
    },
    Chocolate {
        name: "Bûche grand cru",
        drawing: ["  ____  ", " (____@ ", "  ~~~~  "],
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_every_chocolate_has_its_own_name_and_drawing() {
        let names: BTreeSet<_> = CHOCOLATES.iter().map(|chocolate| chocolate.name).collect();
        let drawings: BTreeSet<_> = CHOCOLATES
            .iter()
            .map(|chocolate| chocolate.drawing)
            .collect();
        assert_eq!(names.len(), CHOCOLATES.len());
        assert_eq!(drawings.len(), CHOCOLATES.len());
    }

    #[test]
    fn test_every_drawing_fills_the_same_box() {
        for chocolate in &CHOCOLATES {
            assert!(!chocolate.name.is_empty());
            for line in chocolate.drawing {
                assert!(line.is_ascii(), "{}", chocolate.name);
                assert_eq!(line.chars().count(), DRAWING_WIDTH, "{}", chocolate.name);
                assert!(
                    line.starts_with(' ') && line.ends_with(' '),
                    "{}",
                    chocolate.name
                );
            }
        }
    }
}
