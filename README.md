# budgeters

A terminal-based app for managing your personal finances. Written in Rust

## structure

built from three "tables"
-- Account (acc), made of one or more of the <Account> type
-- Category (cat), made of one or more of the <Category> type
-- Transaction (tra), made of one or more of the <Transaction> type
    * NOTE: can also have <Transfer> type to represent passing between your own <Accounts>

## commands

a <type> : add an <Account>, <Category>, <Transaction>
e <type> : initiate edit method for <type>
d <type> : initiate delete script for<type>
    * NOTE: if <type> is <Account>/<Category>, any <Transaction> will still show that entry
l <type> : list the table for the <type>
/ <query> : search Transaction table by the <string>
--roll <month> <year> <month> <year> : rolls the Table for the first <month>/<year> pair into the second
q : quits the app and saves the files into the correct subdirectory
--cancel : quits the app and does not save any updates
