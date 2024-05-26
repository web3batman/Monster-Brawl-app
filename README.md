# Monster Brawl

The "Monster Brawl" app is an interactive game where different monsters with unique statistics, such as attack and defense, face each other. The project includes an API that allows the management of monsters and battles, offering an exciting and dynamic experience.

## Monster Brawl Algorithm

For calculating the battle algorithm, take into account the flow below:

* The monster with the highest speed makes the first attack, if both speeds are equal, the monster with the higher attack goes first.
* For calculating the damage, subtract the defense from the attack `(attack - defense)`; the difference is the damage; if the attack is equal to or lower than the defense, the damage is 1.
* Subtract the damage from the HP `(HP = HP - damage)`.
* Monsters will battle in turns until one wins; all turns should be calculated in the same request; for that reason, the battle endpoint should return winner data in just one call.
* Who wins the battle is the monster who subtracted the enemy’s HP to zero

## Setup

```bash
docker-compose -f docker-postgres.yaml up -d
diesel setup
diesel migration run
```
