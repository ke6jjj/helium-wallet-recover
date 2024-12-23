# Helium Wallet Rescue

A utility for recovering a lost Helium wallet by exhaustively
trying to correct plausible transcription errors that were made
when writing down the wallet's seed phrase.

The recovery method tries exhaustive combinations of these mistakes:

1. A word is misspelled by changing single letter.
2. A word has an extra letter added to it.
3. A word has a letter removed from it
4. Two adjacent letters in the word have been swapped.
5. The word list was written down row-wise when it should
   have been written down column-wise.
6. The word list was written down column-wise, but with the
   columns inadvertantly swapped.

# Building

TBD. This is a rust project and can be built with `cargo build --release`.

# Usage

At any time use `-h` or `--help` to get more help for the command. In general,
execute the command and provide a starting 12-word seed phrase on the command
line.

Example:

```
./target/release/helium-wallet-recover abandon benefit clever divide fetch hedgehog lend next pride save stem tunnel
```

This will run and print out all valid seed phrase combinations that the utility
can find by trying different spelling and orderings of the phrase.

## Options

Options, if specified, must _precede_ all other command line arguments.

* `-v` / `--verbose` will increase what is printed during the search, showing
   status messages about internal calculation steps during startup as
   well as all seed phrase combinations that were tried, even if they don't
   produce a valid seed phrase checksum.

* `-n` / `--dry-run` will print the number of combinations that will be
   attempted from the given seed phrase and then exit without making any
   attempts. (Can be useful to get a sense of how complex the search job
   will be).

* `-d` will change the spelling distance limit, allowing for wilder spelling
  mistakes, but at a cost of significantly increasing the search time,
  possibly beyond the lifetime of the user. Use `-n` to see the consequences
  of any attempt before attempt in it.

* `-t` / `--target` will tell the tool the what the target public key is,
  allowing it to stop immediately once it is found, and significantly reducing
  the printed output and fruitless search time (if found).

  This option is only useful if the user knows the Solana B58 form of the public
  key for the wallet, which might be known if the user happens to remember
  the name of their Hotspot and can find the public key of its owning
  wallet, for example.

## Misspelling Suggestions

The following table is a list of misspellings of BIP-39 seed phrase words
that themselves are also valid words. Use this list to come up with
alternative words to use when you're having trouble restoring a
crypto wallet from backup.

|Word|Alternatives|
|---|---|
|able|cable table|
|across|cross|
|act|art cat pact|
|action|auction|
|add|dad|
|again|gain|
|age|cage page wage|
|ahead|head|
|aim|air arm|
|air|aim hair pair|
|all|ball call fall ill wall|
|alley|valley|
|alter|later|
|anger|danger|
|angle|ankle|
|ankle|angle|
|arch|march|
|area|arena|
|arena|area|
|arm|aim army art farm warm|
|army|arm|
|around|round|
|arrow|narrow|
|art|act arm cart|
|ask|mask task|
|auction|action|
|aunt|hunt|
|avoid|void|
|awake|aware|
|aware|awake|
|away|way|
|bag|bar tag|
|ball|all call fall wall|
|bar|bag car jar|
|base|case|
|battle|cattle|
|beach|bench teach|
|bean|mean|
|belt|best melt|
|bench|beach|
|best|belt nest test west|
|better|bitter butter letter|
|bid|bind bird kid|
|bike|like|
|bind|bid bird blind find kind mind|
|bird|bid bind|
|bitter|better butter|
|blade|blame|
|blame|blade flame|
|blind|bind|
|blue|blur glue|
|blur|blue|
|blush|brush flush slush|
|boat|goat|
|body|boy|
|boil|coil foil oil|
|bone|one tone zone|
|book|cook|
|border|order|
|boring|bring|
|boss|toss|
|box|boy fox|
|boy|body box joy toy|
|brain|grain rain train|
|brass|grass|
|brick|brisk trick|
|bridge|ridge|
|brief|grief|
|bright|right|
|bring|boring ring|
|brisk|brick risk|
|broom|room|
|brown|frown|
|brush|blush crush|
|bulb|bulk|
|bulk|bulb|
|bus|busy|
|busy|bus|
|butter|better bitter|
|cable|able table|
|cage|age cake case cave page wage|
|cake|cage case cave lake make|
|call|all ball calm fall wall|
|calm|call palm|
|camp|damp lamp ramp|
|can|car cat fan man scan van|
|cannon|canyon|
|canyon|cannon|
|car|bar can card cart cat jar|
|card|car cart hard yard|
|cart|art car card cat|
|case|base cage cake cash cause cave chase|
|cash|case crash dash wash|
|castle|cattle|
|cat|act can car cart chat fat hat|
|catch|match patch|
|cattle|battle castle|
|cause|case pause|
|cave|cage cake case have pave save wave|
|certain|curtain|
|chair|hair|
|change|charge|
|charge|change|
|chase|case|
|chat|cat hat that what|
|chef|chief|
|chief|chef|
|clap|claw clay clip|
|claw|clap clay law|
|clay|clap claw play|
|click|clock|
|climb|limb|
|clip|clap flip|
|clock|click flock lock|
|cloud|loud|
|coach|couch|
|coast|cost roast toast|
|code|come core|
|coil|boil coin cool foil oil|
|coin|coil corn join|
|come|code core home|
|cook|book cool|
|cool|coil cook pool tool wool|
|coral|moral|
|core|code come corn more|
|corn|coin core horn|
|cost|coast host post|
|couch|coach crouch|
|cover|hover over|
|crack|rack track|
|craft|draft|
|cram|cream|
|crash|cash crush trash|
|cream|cram dream|
|crop|drop|
|cross|across|
|crouch|couch|
|crush|brush crash|
|cry|dry try|
|cube|cute tube|
|curtain|certain|
|cute|cube|
|dad|add day mad sad|
|damp|camp lamp ramp|
|danger|anger|
|daring|during|
|dash|cash dish wash|
|dawn|lawn|
|day|dad dry say way|
|deal|dial real|
|decade|decide|
|decide|decade|
|defy|deny|
|deny|defy|
|derive|drive|
|dial|deal|
|dice|ice nice rice|
|diet|dirt|
|dinner|inner winner|
|dirt|diet|
|dish|dash fish wish|
|dog|fog|
|donkey|monkey|
|donor|door|
|door|donor odor|
|dose|dove nose rose|
|dove|dose love move|
|draft|craft drift|
|draw|raw|
|dream|cream|
|drift|draft|
|drip|drop trip|
|drive|derive|
|drop|crop drip|
|dry|cry day try|
|during|daring|
|dust|just must|
|earn|learn|
|east|easy vast|
|easy|east|
|edit|exit|
|eight|height light night right sight|
|either|neither|
|emerge|merge|
|emotion|motion|
|enable|unable|
|enact|exact|
|end|lend|
|estate|state|
|exact|enact|
|exist|exit|
|exit|edit exist|
|face|fade fame race|
|fade|face fame|
|fall|all ball call wall|
|fame|face fade flame frame game name same|
|fan|can fat fun man van|
|farm|arm firm warm|
|fat|cat fan fit flat hat|
|father|gather rather|
|fault|vault|
|fee|feed feel few flee|
|feed|fee feel need seed|
|feel|fee feed fuel|
|fever|never|
|few|fee|
|file|film fine fire|
|film|file firm|
|find|bind fine kind mind|
|fine|file find fire wine|
|finger|ginger|
|fire|file fine firm hire wire|
|firm|farm film fire|
|fish|dish wish|
|fit|fat fix kit|
|fitness|witness|
|fix|fit fox mix six|
|flag|flat|
|flame|blame fame frame|
|flash|flush|
|flat|fat flag float|
|flee|fee|
|flight|light slight|
|flip|clip|
|float|flat|
|flock|clock lock|
|flush|blush flash slush|
|fog|dog fox frog|
|foil|boil coil oil|
|fold|food gold hold old|
|follow|hollow|
|food|fold foot good hood wood|
|foot|food|
|fork|work|
|found|round sound|
|fox|box fix fog|
|frame|fame flame|
|frog|fog|
|front|frost|
|frost|front|
|frown|brown|
|fuel|feel|
|fun|fan gun run sun|
|funny|sunny|
|gain|again grain main rain|
|game|fame gate gaze name same|
|gap|gas gasp|
|garage|garbage|
|garbage|garage|
|gas|gap gasp|
|gasp|gap gas wasp|
|gate|game gaze rate|
|gather|father rather|
|gaze|game gate maze|
|ghost|host|
|giant|grant|
|gift|lift|
|ginger|finger|
|give|live|
|glass|grass|
|glide|guide slide|
|globe|glove|
|glove|globe love|
|glow|grow slow|
|glue|blue|
|goat|boat|
|gold|fold good hold old|
|good|food gold hood wood|
|gown|own town|
|grace|grape race|
|grain|brain gain rain train|
|grant|giant grunt|
|grape|grace|
|grass|brass glass|
|great|treat|
|grid|grit|
|grief|brief|
|grit|grid|
|grow|glow|
|grunt|grant|
|guide|glide|
|gun|fun run sun|
|hair|air chair pair|
|hand|hard sand|
|hard|card hand yard|
|hat|cat chat fat that what|
|have|cave pave save wave|
|head|ahead|
|health|wealth|
|height|eight|
|hen|pen ten then when|
|hero|zero|
|hill|ill pill will|
|hint|hunt|
|hip|ship tip whip|
|hire|fire wire|
|hold|fold gold hole hood old|
|hole|hold home hope pole|
|hollow|follow|
|home|come hole hope|
|hood|food good hold wood|
|hope|hole home|
|horn|corn|
|host|cost ghost post|
|hover|cover over|
|humble|tumble|
|hunt|aunt hint hurt|
|hurt|hunt|
|ice|dice nice rice|
|icon|iron|
|ill|all hill pill will|
|inject|insect|
|inner|dinner winner|
|insect|inject|
|iron|icon|
|issue|tissue|
|item|stem|
|jar|bar car|
|job|joy|
|join|coin|
|joy|boy job toy|
|just|dust must|
|keen|keep|
|keep|keen|
|kick|sick|
|kid|bid kind kit|
|kind|bind find kid mind|
|kiss|miss|
|kit|fit kid kite|
|kite|kit|
|know|now snow|
|lab|law slab|
|lady|lazy|
|lake|cake like make|
|lamp|camp damp ramp|
|later|alter layer water|
|law|claw lab lawn raw|
|lawn|dawn law|
|layer|later|
|lazy|lady|
|learn|earn|
|left|lift|
|lend|end lens|
|lens|lend|
|letter|better|
|life|lift like live wife|
|lift|gift left life list|
|light|eight flight night right sight slight|
|like|bike lake life live|
|limb|climb|
|link|pink wink|
|list|lift|
|live|give life like love olive|
|load|loan loud road|
|loan|load|
|local|loyal vocal|
|lock|clock flock sock|
|long|song|
|lottery|pottery|
|loud|cloud load|
|love|dove glove live move|
|loyal|local royal|
|lumber|number|
|lunch|punch|
|mad|dad maid man sad|
|maid|mad mail main|
|mail|maid main rail sail tail|
|main|gain maid mail man rain|
|make|cake lake maze|
|man|can fan mad main mean van|
|march|arch match|
|mask|ask mass task|
|mass|mask miss pass|
|master|matter|
|match|catch march math patch|
|math|match myth path|
|matter|master|
|maze|gaze make|
|mean|bean man meat|
|meat|mean melt seat|
|medal|metal|
|melt|belt meat|
|mercy|merry|
|merge|emerge|
|merry|mercy|
|metal|medal|
|milk|silk|
|mind|bind find kind|
|miss|kiss mass|
|mix|fix six|
|monkey|donkey|
|moon|soon|
|moral|coral|
|more|core move|
|mother|other|
|motion|emotion|
|move|dove love more movie|
|movie|move|
|much|such|
|mule|rule|
|must|dust just|
|myth|math|
|name|fame game same|
|narrow|arrow|
|near|pear wear year|
|need|feed seed|
|neither|either|
|nest|best net next test west|
|net|nest next nut pet wet|
|never|fever|
|next|nest net text|
|nice|dice ice rice|
|night|eight light right sight|
|noise|nose|
|north|worth|
|nose|dose noise note rose|
|note|nose vote|
|now|know snow|
|number|lumber|
|nurse|purse|
|nut|net put|
|odor|door|
|oil|boil coil foil|
|old|fold gold hold|
|olive|live|
|once|one|
|one|bone once tone zone|
|open|oven pen|
|orange|range|
|order|border|
|other|mother|
|oven|open over|
|over|cover hover oven|
|own|gown town|
|ozone|zone|
|pact|act|
|paddle|saddle|
|page|age cage pave wage|
|pair|air hair|
|palace|place|
|palm|calm|
|pass|mass|
|patch|catch match path pitch|
|path|math patch|
|pause|cause|
|pave|cave have page save wave|
|peace|place|
|pear|near wear year|
|pen|hen open pet ten|
|pet|net pen poet put wet|
|pill|hill ill pull will|
|pink|link wink|
|pitch|patch|
|place|palace peace plate|
|plate|place|
|play|clay|
|poem|poet|
|poet|pet poem post|
|point|print|
|polar|solar|
|pole|hole|
|pond|pony|
|pony|pond|
|pool|cool tool wool|
|post|cost host poet|
|pottery|lottery|
|powder|power|
|power|powder tower|
|praise|raise|
|present|prevent|
|prevent|present|
|price|pride prize rice|
|pride|price prize ride|
|print|point|
|prize|price pride|
|project|protect|
|proof|roof|
|protect|project|
|pull|pill pulp|
|pulp|pull|
|pulse|purse|
|punch|lunch|
|purse|nurse pulse|
|put|nut pet|
|quit|quiz suit|
|quiz|quit|
|race|face grace rack rare rate rice|
|rack|crack race track|
|rail|mail rain sail tail|
|rain|brain gain grain main rail train|
|raise|praise|
|ramp|camp damp lamp|
|range|orange|
|rare|race rate|
|rate|gate race rare|
|rather|father gather|
|raw|draw law|
|real|deal|
|reason|season|
|rent|tent|
|rice|dice ice nice price race rich ride|
|rich|rice|
|ride|pride rice ridge rude side tide wide|
|ridge|bridge ride|
|right|bright eight light night sight|
|ring|bring sing wing|
|risk|brisk|
|road|load|
|roast|coast toast|
|roof|proof room|
|room|broom roof|
|rose|dose nose|
|round|around found sound|
|royal|loyal|
|rude|ride rule|
|rug|run|
|rule|mule rude|
|run|fun gun rug sun|
|sad|dad mad sand say|
|saddle|paddle|
|safe|same save|
|sail|mail rail tail|
|salmon|salon|
|salon|salmon|
|same|fame game name safe save|
|sample|simple|
|sand|hand sad stand|
|save|cave have pave safe same wave|
|say|day sad shy spy stay way|
|scale|scare|
|scan|can|
|scare|scale share spare|
|sea|seat|
|season|reason|
|seat|meat sea|
|seed|feed need seek shed speed|
|seek|seed|
|sell|shell spell tell|
|shaft|shift|
|shallow|swallow|
|share|scare spare|
|shed|seed|
|shell|sell spell|
|shift|shaft swift|
|ship|hip shop whip|
|shock|sock stock|
|shoe|shop shove|
|shoot|short|
|shop|ship shoe|
|short|shoot sort sport|
|shove|shoe stove|
|shy|say spy|
|sick|kick silk sock stick|
|side|ride size slide tide wide|
|sight|eight light night right slight|
|sign|sing|
|silk|milk sick|
|simple|sample|
|sing|ring sign song sting swing wing|
|six|fix mix|
|size|side|
|skate|state|
|ski|skin|
|skill|skull still|
|skin|ski spin|
|skull|skill|
|slab|lab slam|
|slam|slab slim|
|slice|slide spice|
|slide|glide side slice|
|slight|flight light sight|
|slim|slam swim|
|slot|slow spot|
|slow|glow slot snow|
|slush|blush flush|
|smart|start|
|snap|soap swap|
|snow|know now slow|
|soap|snap soup swap|
|sock|lock shock sick stock|
|soft|sort|
|solar|polar|
|song|long sing|
|soon|moon spoon|
|sorry|worry|
|sort|short soft sport|
|soul|soup|
|sound|found round|
|soup|soap soul|
|south|youth|
|space|spare spice|
|spare|scare share space|
|speak|steak|
|speed|seed spend|
|spell|sell shell|
|spend|speed|
|spice|slice space spike|
|spike|spice|
|spin|skin|
|spoon|soon|
|sport|short sort spot|
|spot|slot sport|
|spy|say shy|
|stable|table|
|staff|stuff|
|stage|state|
|stamp|swamp|
|stand|sand|
|start|smart|
|state|estate skate stage|
|stay|say|
|steak|speak|
|stem|item step|
|step|stem|
|stick|sick stock|
|still|skill|
|sting|sing swing|
|stock|shock sock stick|
|stone|stove tone|
|stool|tool|
|stove|shove stone|
|stuff|staff|
|stumble|tumble|
|such|much|
|suit|quit|
|sun|fun gun run|
|sunny|funny|
|sure|surge|
|surge|sure urge|
|swallow|shallow|
|swamp|stamp swap|
|swap|snap soap swamp|
|swarm|warm|
|swear|wear|
|swift|shift|
|swim|slim|
|swing|sing sting wing|
|sword|word|
|table|able cable stable|
|tag|bag|
|tail|mail rail sail|
|talk|tank task walk|
|tank|talk task thank|
|tape|type|
|task|ask mask talk tank|
|taste|waste|
|teach|beach|
|team|term|
|tell|sell|
|ten|hen pen tent then|
|tent|rent ten test text|
|term|team|
|test|best nest tent text west|
|text|next tent test|
|thank|tank|
|that|chat hat what|
|theme|there|
|then|hen ten they when|
|there|theme three where|
|they|then|
|three|there tree|
|tide|ride side time wide|
|time|tide|
|tip|hip top trip|
|tissue|issue|
|toast|coast roast|
|toe|tone top toy|
|tone|bone one stone toe zone|
|tool|cool pool stool wool|
|top|tip toe toy|
|toss|boss|
|tower|power|
|town|gown own|
|toy|boy joy toe top try|
|track|crack rack trick truck|
|train|brain grain rain|
|trap|tray trip wrap|
|trash|crash|
|tray|trap try|
|treat|great|
|tree|three true|
|trick|brick track truck|
|trim|trip|
|trip|drip tip trap trim|
|truck|track trick|
|true|tree|
|try|cry dry toy tray|
|tube|cube|
|tumble|humble stumble|
|twin|win|
|type|tape|
|unable|enable|
|urge|surge|
|use|used|
|used|use|
|valley|alley|
|van|can fan man|
|vast|east|
|vault|fault|
|verb|very|
|very|verb|
|vocal|local|
|void|avoid|
|vote|note|
|wage|age cage page wave|
|wait|want|
|walk|talk wall|
|wall|all ball call fall walk will|
|want|wait|
|warm|arm farm swarm|
|wash|cash dash wasp wish|
|wasp|gasp wash|
|waste|taste|
|water|later|
|wave|cave have pave save wage|
|way|away day say|
|wealth|health|
|wear|near pear swear year|
|web|wet|
|west|best nest test wet|
|wet|net pet web west|
|what|chat hat that wheat|
|wheat|what|
|when|hen then|
|where|there|
|whip|hip ship|
|wide|ride side tide wife wine wire wise|
|wife|life wide wine wire wise|
|wild|will|
|will|hill ill pill wall wild|
|win|twin wine wing wink|
|wine|fine wide wife win wing wink wire wise|
|wing|ring sing swing win wine wink|
|wink|link pink win wine wing|
|winner|dinner inner winter|
|winter|winner|
|wire|fire hire wide wife wine wise|
|wise|wide wife wine wire wish|
|wish|dish fish wash wise|
|witness|fitness|
|wood|food good hood wool word|
|wool|cool pool tool wood|
|word|sword wood work world|
|work|fork word|
|world|word|
|worry|sorry|
|worth|north|
|wrap|trap|
|yard|card hard|
|year|near pear wear|
|youth|south|
|zero|hero|
|zone|bone one ozone tone|

## Technical Note

This list was generated by finding all words in the
list that were one step away from each other under the
Damerau-Levenshtein metric. You can generate it by running
one of the unit tests inside this package.
