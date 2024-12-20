import { mutation, query } from "./_generated/server";
import { v } from "convex/values";

export const createGame = mutation({
  args: {
    createdBy: v.string(),
  },
  handler: async (ctx, args) => {
    return await ctx.db.insert("games", {
      createdBy: args.createdBy,
      status: "waiting",
      currentEmoji: "",
      players: [args.createdBy],
      scores: { [args.createdBy]: 0 },
    });
  },
});

export const joinGame = mutation({
  args: {
    gameId: v.id("games"),
    playerId: v.string(),
  },
  handler: async (ctx, args) => {
    const game = await ctx.db.get(args.gameId);
    if (!game || game.status !== "waiting") {
      throw new Error("Game not available");
    }
    
    const players = [...game.players, args.playerId];
    const scores = { ...game.scores, [args.playerId]: 0 };
    
    await ctx.db.patch(args.gameId, {
      players,
      scores,
    });
  },
});

export const submitGuess = mutation({
  args: {
    gameId: v.id("games"),
    playerId: v.string(),
    guess: v.string(),
  },
  handler: async (ctx, args) => {
    await ctx.db.insert("guesses", {
      gameId: args.gameId,
      playerId: args.playerId,
      guess: args.guess,
      timestamp: Date.now(),
    });
  },
});

export const getGame = query({
  args: { gameId: v.id("games") },
  handler: async (ctx, args) => {
    return await ctx.db.get(args.gameId);
  },
});
