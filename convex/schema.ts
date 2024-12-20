import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  games: defineTable({
    createdBy: v.string(),
    status: v.string(),
    currentEmoji: v.string(),
    players: v.array(v.string()),
    scores: v.map(v.string(), v.number()),
    winner: v.optional(v.string()),
  }),
  guesses: defineTable({
    gameId: v.id("games"),
    playerId: v.string(),
    guess: v.string(),
    timestamp: v.number(),
  }),
});
