type count = number;

type user = {
   user_id: string;
   words: {
      [word0: string]: {
         total_count: count;
         [word1: string]: count;
      };
   }
}
