# budgeters

A terminal-based app for managing your personal finances. Written in Rust

## structure

built from three "tables"
-- Account (acc), made of one or more of the <Account> type
-- Category (cat), made of one or more of the <Category> type
-- Transaction (tra), made of one or more of the <Transaction> type
    * NOTE: can also have <Transfer> type to represent passing between your own <Accounts>

## commands

a <type> : initiate add method for either <Account>, <Category>, or <Transaction>
e <type> : initiate edit method for <type>
d <type> : initiate delete script for <type>
    * NOTE: if <type> is <Account>/<Category>, any <Transaction> will still show that entry
l <type> : list the table for the <type>
/ <query> : search <Transaction> table by the <string>
--update : update the budget spread (update <Account> value and <Category> actual)
q : quits the app and saves the files into the correct subdirectory
--cancel : quits the app and does not save any updates

## coming soon
s <field> : sort <Transaction> table by the specified <field>
--open : open a year/month budget
--close : close a year/month budget
--roll <month> <year> <month> <year> : rolls the Table for the first <month>/<year> pair into the second
? : describes all available commands
