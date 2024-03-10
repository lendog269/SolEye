﻿## SolEye
SolEye aims to make the internet safer by preventing scams using transformer models as well as the knowledge of the community.
## Checking the Safety of Websites
We utilise a chrome extension in order to check the safety of the website a user visits in real time. The data from a visited website such as: url, site content, js code and domain age, will be sent to the transformer model where it will be evaluated and a safety rating (0-1), as well as reasons why the website may be suspected as unsafe will be provided back to the user. This will appear as a banner that turns either green for a safe website or red for an unsafe website. The user is then able to access the extension in order to see the percentage chance of the website being safe as well as the provided reasons catagorized as either: url, site content, js code or domain age. The website url will then be stored on either the whitelist or blacklist held on the blockchain. Over time, as more websites are accessed using the extension the whitelist and blacklist will be built up. This means that instead of re-evaluating a website each time, the extension will instead first check the stored whitelist and blacklist and display the stored result to the user, speeding up the checking process massively.  
## Voting
If a user believes a website has been wrongly classified and stored on the wrong list, they are able to begin a community vote in order to change the list the website is stored on. Beginning a vote will cost tokens from our own supply with the amount it costs scaling with the safety rating given by the transformer model. This cost will scale as 1000 * 2^(-10(x-0.5)), if  0 <= x < 0.5 or 1000 * 2^(10(x-0.5)), if  0.5 <= x <=1. This is the same graph mirrored about a safety rating of 0.5 meaning it will cost the same amount of tokens to start a vote to change the classification of a website given a 0 safety rating as a website given a 1 safety rating. Votes will begin with a with a duration of 24h and one second will be added to the timer for each token used to vote. When the vote expires the website will be put onto the list with more votes. Users who voted for the correct side will have their tokens returned to them and the tokens used to vote on the wrong side will be kept. Of these tokens some will be distributed to the users who voted for the correct side as a reward, some will be kept in order to make it profitable, and some will be burnt. This actively rewards good actors and slashes bad actors making the system more trustworthy. If a user wishes to start a vote on a website which has been voted on before, the cost of starting this vote will be the same as the number of tokens that won the last vote.
## Token Distribution
Tokens would initially be distributed as follows: 40% to the community and ecosystem (a large portion of this would be allocated to trusted actors), 10% to marketing and partnerships, 15% to research and development, 15% to the foundation reserve, and 20% to the team and advisors. This distribution would allow for votes to be trustworthy and ensure the safety of the community.
## The Future
In order to best utilise the community the transformer model will be trained based on the results of the community votes as this will bring the transformer model more in line with the community and more likely to be correct in its judgement the first time around.