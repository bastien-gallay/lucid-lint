The migration added a caching layer in front of the database. Latency spiked under heavy writes. Cold keys triggered a thundering-herd pattern. We coalesced the fills. Jittered TTLs ended the spikes.
