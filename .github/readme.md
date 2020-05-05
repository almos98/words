# Words
Live version can be found at [words.ratex.dev](https://words.ratex.dev)

## API
| TYPE | ROUTE | DESCRIPTION |
| ---- | ----- | ----------- |
| GET  | /list | Return a list of all stored lists |
| GET  | /list/LIST-NAME | Returns the words that are part of LIST-NAME |
| POST | /list/LIST-NAME | Add words to LIST-NAME |
| GET  | /words          | Returns words stored in ALL lists |
| GET  | /words?list=list1,list2 | Returns words from list1 and list2 |