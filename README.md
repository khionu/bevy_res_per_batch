# Bevy Resource per Batch -- Does not compile

This is what I would imagine a workaround look like for trying to have one lock/atomic-less resource per query batch. Problem is that Mut contains a Cell which means it's not thread safe. Bevy itself has to use unsafe internally to make `par_for_each{,mut}` work, so this isn't terribly surprising that a safe workaround would run into this kind of blocker.
