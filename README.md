Transformer 'like' architecture is written in Rust. The project is in progress, only preliminary results are ready. 
I use bilingual (French English and Finnish English sets at the moment) sentence pairs from the Tatoeba Project: http://www.manythings.org/anki/ to develop and to test the architecture. There are 185583 pairs in French English set, and 63562 pairs in Finnish English set. 

Some main ideas:
1. Firstly tokenizers are running to generate 'basic vectors' -> tokens. 
The tokenizers are running at the level of words and at the level of sentences. That is why the final tokens are the mixture of charactar like tokens like 'ing' , full words like 'her' , 'idioms' like 'of the' , 'her head'. 
All the tokens are out 'features' for classification ('basic vectors').
2. Source sentences are split on the tokens.
3. Let us consider a first word in target sentence: 'word1'. There are several sentences which has the 'word1' as the first word in translation. 'word1' is category in classification and tokens in the sentences are the evidence features for the category. We use first words in target language as categories and tokens in source language as features for categorization. 
4. The first task is: by the features (tokens) in source language predict the category -> the first word in the target sentence.
5. After the first word prediction, we will use the tokenized version of the word (tokenized in the target language, of course) as the set of additional features to predict the second word in target sentence. The tokens (features) of the source sentence are combined with tokens from the just predicted first word in target sentence. Using the set we predict the second word in target sentence, split it on tokens, add the tokens to the previous set and use it to predict the third word.....etc.... The recursive process resembles what is done in case of the Tokenizer and RNN, that is why I use 'Tokenizer like' term.
6. I use Bayesian methods for the categories (which are 'words' in the case).
Why Bayesian. 
a. the target is try not to use very computationally expensive 'back propagation' optimization methods.  
b. Bayesian methods have very clear theoretical basis, we may easily generate and try different architecture modifications by using the basis. 
c. we are very close to 'language' intuition in the case.
d. it is relatively easy to combine differen features together. for 
example in the first part of the work I try to unsver the question: how good it is possible to predict the first (second, ...third...) word in target sentence 
by using just the 'length' of source sentence as predictor ? The 'length' of source sentence is a metadata , and we can easily combine such meta-features with token-features.

The code is organized as a pipeline of crates, the pipelines are quite natural in NLP. The results of crates computations are serialized and saved in .json files (using serde in Rust) and deserialized as the initial data in the next crate. There is also fuctionality to save the intermediate computations in Postgres DB.  

