actor Counter {

  stable var counter = 0;

  // Get the value of the counter.
  public query func get() : async Nat {
    return counter;
  };

  public query func add(n: Nat) : async Nat {
      return counter + n;
  };

  public shared(msg) func msgCaller(): async Principal {
      return msg.caller;
  };


  // Set the value of the counter.
  public func set(n : Nat) : async () {
    counter := n;
  };

  // Increment the value of the counter.
  public func inc() : async () {
    counter += 1;
  };

};
