import React, { useEffect, useState } from 'react';
import { useMutation, useQuery } from 'convex/react';
import { api } from '../../convex/_generated/api';
import { Id } from '../../convex/_generated/dataModel';
import { EmojiGame as WhoEmojiGame } from '@who/emoji-is-it';
import { Box, Button, Typography } from '@mui/material';

interface EmojiGameProps {
  gameId: Id<'games'>;
  playerId: string;
}

export const EmojiGame: React.FC<EmojiGameProps> = ({ gameId, playerId }) => {
  const game = useQuery(api.games.getGame, { gameId });
  const submitGuess = useMutation(api.games.submitGuess);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    if (game && game.status === 'waiting') {
      setIsReady(true);
    }
  }, [game]);

  const handleGuess = async (guess: string) => {
    await submitGuess({ gameId, playerId, guess });
  };

  if (!game) {
    return <Typography>Loading...</Typography>;
  }

  return (
    <Box sx={{ p: 2 }}>
      <Typography variant="h4" gutterBottom>
        Emoji Game
      </Typography>
      
      {isReady ? (
        <WhoEmojiGame
          gameId={gameId}
          playerId={playerId}
          onGuess={handleGuess}
          apiKey={process.env.REACT_APP_WHO_EMOJI_API_KEY}
        />
      ) : (
        <Typography>Waiting for players...</Typography>
      )}
      
      <Box sx={{ mt: 2 }}>
        <Typography variant="h6">Players:</Typography>
        {game.players.map((player) => (
          <Typography key={player}>
            {player}: {game.scores[player]} points
          </Typography>
        ))}
      </Box>
    </Box>
  );
};
