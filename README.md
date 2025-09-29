# `tetra-mac`

Toy/sim environment for playing with the TETRA V+D (ETSI EN 300 392-2) MAC (mainly).

## Functionality

* Physical
  * Burst build/extract; phase adjustment
  * π/4 DQPSK mod (toy)
  * Synchroniser (in progress)
* Lower MAC
  * Error control structures
    * RM code (for AACH) / Block (CRC) code (for CCH)
    * Rate-Compatible Punctured Convolutional Code + Viterbi decode (currently only for CCH).
    * Interleaving (currently only 1-block)
    * Scrambling
* Upper MAC
  * Protocol PDU encode/decode (still working on less-used structures)