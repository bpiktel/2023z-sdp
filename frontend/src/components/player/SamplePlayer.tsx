"use client";
import { MutableRefObject } from "react";
import { Howl } from "howler";
import { PlayButton, StopButton } from "./ControlButtons";

const SamplePlayer = ({
  assetPath,
  name,
  playerRef,
  className,
  status,
  setStatus
}: {
  assetPath: string;
  name: string;
  playerRef: MutableRefObject<Howl | undefined>;
  className?: string;
  status: string | null;
  setStatus: React.Dispatch<React.SetStateAction<string | null>>;
}): JSX.Element => {
  const setNewStatus = (newStatus: string | null) => {
    if (newStatus === status)
      return

    if (status === name && newStatus === null) {
      playerRef.current?.stop();
      setStatus(newStatus);
    }

    if (newStatus !== null) {
      playerRef.current?.stop();
      playerRef.current = new Howl({
          src: [assetPath],
          volume: 1,
          loop: false,
          html5: true,
          autoplay: true,
          format: ['mp3']
        }
      );
      setStatus(newStatus);
    }
  }


  return (
    <div className={`flex flex-col items-center min-w-[16rem] ${className}`}>
      <div className="text-md font-semibold">{name}</div>
      <div className="flex gap-sm mt-xs justify-center">
        <PlayButton
          onClick={() => {
            setNewStatus(name);
          }}
        />
        <StopButton
          disabled={name !== status}
          onClick={() => {
            setNewStatus(null);
          }}
        />
      </div>
    </div>
  );
};

export default SamplePlayer;
