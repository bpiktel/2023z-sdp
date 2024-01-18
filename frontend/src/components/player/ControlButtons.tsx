import { FaStop, FaPlay } from "react-icons/fa";

export const PlayButton = ({
  onClick,
}: {
  onClick: () => void;
}): JSX.Element => {
  return (
    <div data-testid="play-button">
      <ControlButton label={<FaPlay />} onClick={onClick} />
    </div>
  );
};

export const StopButton = ({
  onClick,
  disabled,
}: {
  onClick: () => void;
  disabled?: boolean;
}): JSX.Element => {
  return (
    <div data-testid="stop-button">
      <ControlButton label={<FaStop />} onClick={onClick} disabled={disabled} />
    </div>
  );
};

const ControlButton = ({
  label,
  onClick,
  disabled,
}: {
  label: JSX.Element;
  onClick: () => void;
  disabled?: boolean;
}): JSX.Element => {
  return (
    <button
      className={`${
        disabled ? "bg-gray-600 cursor-default" : "bg-blue-600"
      } text-white rounded-md h-8 w-8 p-2.5 flex items-center justify-center`}
      onClick={onClick}
    >
      {label}
    </button>
  );
};
