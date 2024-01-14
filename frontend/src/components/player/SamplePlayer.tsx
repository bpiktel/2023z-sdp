"use client";
import { useEffect, useRef, useState } from "react";
import { Howl } from "howler";
import { PauseButton, PlayButton, StopButton } from "./ControlButtons";

const SamplePlayer = ({
  assetPath,
  name
}: {
  assetPath: string;
  name: string;
}): JSX.Element => {
  const playerRef = useRef<Howl>(
    new Howl({
      src: [assetPath],
      volume: 1,
      loop: false,
      html5: true,
      format: ['mp3']
    })
  );

  const [status, setStatus] = useState<"stopped" | "playing" | "paused">(
    "stopped"
  );

  useEffect(() => {
    const player = playerRef.current;

    switch (status) {
      case "playing":
        player.play();
        break;
      case "paused":
        player.pause();
        break;
      case "stopped":
        player.stop();
        break;
    }

    return () => {
      player.stop();
    };
  }, [playerRef, status]);

  return (
    <div className="flex flex-col items-center min-w-[16rem]">
      <div className="text-md font-semibold">{name}</div>
      <div className="flex gap-sm justify-center">
        <PlayButton
          onClick={() => {
            setStatus("playing");
          }}
        />
        <PauseButton
          onClick={() => {
            setStatus("paused");
          }}
        />
        <StopButton
          onClick={() => {
            setStatus("stopped");
          }}
        />
      </div>
    </div>
  );
};

export default SamplePlayer;
