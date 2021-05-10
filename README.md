Transformer 'like' architecture is written in Rust. The project is in progress, only preliminary results are ready. 
I use bilingual (French English and Finnish English sets at the moment) sentence pairs from the Tatoeba Project: http://www.manythings.org/anki/ to develop and to test the architecture. There are 185583 pairs in French English set, and 63562 pairs in Finnish English set. 
Instead of a neural networks layers I try to use Bayesian approach for target words prediction. 

Some main ideas:
1. Firstly tokenizers are running to generate 'basic vectors' (features) -> tokens. 
The tokenizers are running at the level of words and at the level of sentences. That is why the final tokens are the combination of tokens like 'ing' , full words like 'her' , and 'idioms' like 'of the' , 'her head' ... etc. 
The tokens are the 'features' for classification ('basic vectors').
2. Source sentences are split to the tokens. 
3. Let us consider a first word in target sentence: 'word1'. In training set there are several source sentences which has the 'word1' as the first word in translation. 'word1' is category in classification and tokens in the sentences are the evidence features for the category. We use first words in target language as categories and tokens in source language as features for categorization. 
4. The first task is: by the features (tokens) in source language predict the category -> the first word in the target sentence.
5. After the first word prediction, we will use the tokenized version of the word (tokenized in the target language, of course) as the set of additional features to predict the second word in target sentence. The tokens (features) of the source sentence are combined with tokens from the just predicted first word in target sentence. Using the set we predict the second word in target sentence, split it on tokens, add the tokens to the previous set and use it to predict the third word.....etc.... The recursive process resembles what is done in case of the Tokenizer and RNN, that is why I use 'Tokenizer like' term.
6. I use Bayesian methods for the categories (which are 'words' in the case).
Why Bayesian. 
a. the target is try not to use very computationally expensive 'back propagation'.  
b. Bayesian methods have very clear theoretical basis and intuition, we may easily generate and try different architecture modifications by using the basis. 
d. it is relatively easy to combine differen features and stack submodels.
7. Words embedding. It will be need to write a words embedding from scratch to 
have possibiity to use more words than there are in the translation pairs. 

The code is organized as a pipeline of crates. The results of crates computations are serialized and saved in .json files (using serde in Rust) and deserialized as the initial data in the next crate. There is also fuctionality to save the intermediate computations in Postgres DB.  

