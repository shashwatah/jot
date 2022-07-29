# jot

##### commands:
```bash
<SUBCOMMANDS>
jot vlt 
jot nts
jot dir 
jot cdr
jot his
jot lst
jot fnd
jot mem

<USAGE>
jot vlt                                     # list all and active vault
jot vlt test *location*				        # create vault
jot vlt en test						        # enter vault
jot vlt del test						    # delete vault
jot vlt ren test test2				        # rename vault
jot vlt mov test *new location*			    # move vault to a different location

jot nts 							        # list all notes in current dir
jot nts "note"						        # create a note
jot nts op "note"					        # open note
jot nts ren "note" "new name"			    # rename note
jot nts del "note"					        # delete note	
jot nts mov "note" *new location*		    # move note

jot dir							            # get vaults directory tree highlighting the current dir
jot dir test						        # create directory 
jot dir del *path to dir*				    # delete dir 
jot dir ren *path to dir* *path to dir*	    # rename dir 
jot cdr *path to dir*					    # change dir

jot his 							        # get history
jot his op 						            # get selectable list 

jot lst							            # open last note

jot fnd "query" 						    # search in current vault

jot mem							            # list memos
jot mem "adasdsadasdasd"			        # create memo
jot mem del						            # delete memo
```

