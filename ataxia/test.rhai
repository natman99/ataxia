name = "hello 2 ";
classes[0].set_class("barbarian");
health.max = 26;

// saving_throws.str = true;
//
skills.add("acrobatics");

ability_scores.dex = 20;
skills.proficiency_bonus = 3;

let i = item("test", "cool item");

inventory.add(i);

let i = item("cool item", "cool item");
i.feature(Str, 2);
i.roll("2d6 + " + ability_scores.str.mod());

let s = "2d6 + " + ability_scores.dex.mod();
i.roll(s);

i.to_hit = Dex;

meters.add(meter("test", 4));

inventory.add(i);
