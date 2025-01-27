
use std::collections::HashMap;

trait CreateNew {
    fn create_new(self) -> HashMap<String, String>;
}

impl CreateNew for HashMap<&str, &str> {
    fn create_new(self) -> HashMap<String, String>  {
        let mut new_map = HashMap::new();
        for (key, value) in self.clone() {
            new_map.insert(String::from(key), String::from(value));
        };
        return new_map
    }
}

fn main() {

    let binders = HashMap::from([
        ("cDPW", "Dispel Magic"),
        ("cSWWS", "Summon Ice Elemental"),
        ("cWSSW", "Summon Fire Elemental"),
        ("cw", "Magic Mirror"),
        ("DFFDD", "Lightning Bolt"),
        ("DFPW", "Cure Heavy Wounds"),
        ("DFW", "Cure Light Wounds"),
        ("DFWFd", "Blindness"),
        ("DPP", "Amnesia"),
        ("DSF", "Confusion/Maladroitness"),
        ("DSFFFc", "Disease"),
        ("DWFFd", "Blindness"),
        ("DWSSSP", "Delay Effect"),
        ("DWWFWD", "Poison"),
        ("FFF", "Paralysis"),
        ("FPSFW", "Summon Troll"),
        ("FSSDD", "Fireball"),
        ("P", "Shield"),
        ("PDWP", "Remove Enchantment"),
        ("PPws", "Invisibility"),
        ("PSDD", "Charm Monster"),
        ("PSDF", "Charm Person"),
        ("PSFW", "Summon Ogre"),
        ("PWPFSSSF", "Finger of Death"),
        ("PWPWWc", "Haste"),
        ("SD", "Magic Missile"),
        ("SFW", "Summon Goblin"),
        ("SPFP", "Anti-spell"),
        ("SPFPSDW", "Permanency"),
        ("SPPc", "Time Stop"),
        ("SPPFD", "Time Stop"),
        ("SSFP", "Resist Cold"),
        ("SWD", "Fear (No CFDS)"),
        ("SWWc", "Fire Storm"),
        ("WDDc", "Clap of Lightning"),
        ("WFP", "Cause Light Wounds"),
        ("WFPSFW", "Summon Giant"),
        ("WPFD", "Cause Heavy Wounds"),
        ("WPP", "Counter Spell"),
        ("WSSc", "Ice Storm"),
        ("WWFP", "Resist Heat"),
        ("WWP", "Protection"),
        ("WWS", "Counter Spell"),
    ]);

    for (sign, spell) in binders.clone().create_new().iter() {
        let binder = binders.clone().create_new();
        find_chain(String::from(sign), String::from(spell), binder);
    }
}

// starting from last letter, concatenate progressively larger strings
// and match with links at beginning of the other spells
fn find_chain(mut sign: String, spell: String, binder: HashMap<String, String>) {

    let cap: usize = sign.len();

    for n in 0..cap-1 {
        let sub = sign.get_mut(n..cap).map(|s| &*s).unwrap();

        // Print title
        if n == 0 { println!("{}   {}", sub, spell) };

        for (acronym, spell_name) in binder.clone().iter() {
            if acronym.starts_with(sub) {
                if String::from(spell_name).ne(&spell) && sub != acronym {
                    println!("  > {}: {}", acronym, spell_name);
                }
            }
        }
    }
    println!("");
}