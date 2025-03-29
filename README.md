# Escrow
This simple smart contract allows you to create and finalize payments. When payment is initialized, a `Condition` and `Lockup` accounts are created. Condition (String) is written into the `Condition` account, while funds are deposited into the `Lockup` account.

Then, after `payer` marks the condition as passed, they can invoke `finalize_payment()` functionality, closing and reclaiming the `Condition` account rent, and unlocking funds from `Lockup`, then transferring them to the payment recipient.

## Changes since the lesson
After the lesson, I've written two tests that you can see in the `/tests` directory. During testing, some account mutability issues were detected (which is why testing is so important!) and fixed in the contract. The contract may slightly vary from the version written live, during the lesson.

In the second step of the escrow flow (`finalize_payment` instruction), funds are transferred from `Lockup` account to the `recipient`. This requires us to use the `Lockup` account as a signer (`CpiContext::new_with_signer`) - since the account must authorize changes in it's own state. I somehow skipped this part during the lesson. 

We'll come back and explain all changes at the start of the next Bootcamp lesson, so you don't miss anything.
