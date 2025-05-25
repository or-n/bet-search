import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/result

pub type Event {
  Event(tag: Tag, time: Time, side: Side, player: Option(String))
}

pub type SumEvent {
  Sum(Tag)
  From(SumEvent, Time)
  To(SumEvent, Time)
  Side(SumEvent, side: Side)
  Player(SumEvent, String)
}

pub type BoolEvent {
  Less(SumEvent, SumEvent)
  Equal(SumEvent, SumEvent)
  Or(BoolEvent, BoolEvent)
}

pub type Time {
  Time(minute: Int, second: Int)
}

pub type Tag {
  ShotStart
  ShotEnd(ShotEnd)
  Corner
  Foul
  YellowCard
  RedCard
}

pub type ShotEnd {
  OffTarget
  Blocked
  Saved
  Goal
}

pub type Side {
  Home
  Away
}

pub const start = Time(0, 0)

pub const break = Time(45, 0)

pub const end = Time(90, 0)

pub const goals = Sum(ShotEnd(Goal))

pub const goals1 = Side(goals, Home)

pub const goals2 = Side(goals, Away)

pub const goals_h1 = To(goals, break)

pub const goals_h2 = From(goals, break)

pub const winner1 = Less(goals2, goals1)

pub const more_goals_h1 = Less(goals_h2, goals_h1)

pub fn eval(events: List(Event), expr) {
  case expr {
    Less(a, b) -> {
      use a_value <- result.try(eval_sum(events, a))
      use b_value <- result.try(eval_sum(events, b))
      Ok(list.length(a_value) < list.length(b_value))
    }
    Equal(a, b) -> {
      use a_value <- result.try(eval_sum(events, a))
      use b_value <- result.try(eval_sum(events, b))
      Ok(list.length(a_value) == list.length(b_value))
    }
    Or(a, b) -> {
      use a_value <- result.try(eval(events, a))
      use b_value <- result.try(eval(events, b))
      Ok(a_value || b_value)
    }
  }
}

pub fn time_from(time: Time, x: Time) {
  case time.minute == x.minute {
    True -> time.second >= x.second
    _ -> time.minute >= x.second
  }
}

pub fn time_to(time: Time, x: Time) {
  case time.minute == x.minute {
    True -> time.second <= x.second
    _ -> time.minute <= x.second
  }
}

pub fn eval_sum(events: List(Event), expr) {
  case expr {
    Sum(tag) ->
      events
      |> list.filter(fn(x) { x.tag == tag })
      |> Ok
    From(x, time) -> {
      use events <- result.try(eval_sum(events, x))
      events
      |> list.filter(fn(x: Event) { x.time |> time_from(time) })
      |> Ok
    }
    To(x, time) -> {
      use events <- result.try(eval_sum(events, x))
      events
      |> list.filter(fn(x: Event) { x.time |> time_to(time) })
      |> Ok
    }
    Side(x, side) -> {
      use events <- result.try(eval_sum(events, x))
      events
      |> list.filter(fn(x: Event) { x.side == side })
      |> Ok
    }
    Player(x, player) -> {
      use events <- result.try(eval_sum(events, x))
      events
      |> list.filter(fn(x: Event) { x.player == Some(player) })
      |> Ok
    }
  }
}

pub fn main() {
  let events = echo [Event(ShotEnd(Goal), break, Home, None)]
  let _ = echo eval(events, winner1)
  let _ = echo eval_sum(events, goals_h1)
  let _ = echo eval_sum(events, goals_h2)
  let _ = echo eval(events, more_goals_h1)
}
