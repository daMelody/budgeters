# budgeters

a terminal-based app for managing your personal finances

# structure

built from three "tables"
-- Account (acc), made of one or more of the <Account> type
-- Category (cat), made of one or more of the <Category> type
-- Transaction (tra), made of one or more of the <Transaction> type
    * NOTE: can also have <Transfer> type to represent passing between your own <Accounts>

# commands

-a <type>: add an <Account>, <Category>, <Transaction>, or <Transfer>
-e <type> <id> : edit the <type> entry according to the <id>
-d <type> <id> : delete the <type> entry according to the <id>
    * NOTE: if <type> is <Account>/<Category>, any <Transaction> or <Transfer> will still show that entry
-l <type> : list the table for one type
-s <string>: search Transaction table by the <string>
