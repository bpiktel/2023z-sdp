export const ButtonPrimary = ({
  children,
  className,
  onClick,
  disabled
}: {
  children: JSX.Element | JSX.Element[] | string;
  className?: string;
  onClick?: () => void;
  disabled?: boolean;
}) => {
  return (
    <button
      className={`bg-primary hover:bg-primary-dark px-4 py-2 text-white rounded-md disabled:pointer-events-none ${className}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
};

export const ButtonSecondary = ({
  children,
  className,
  onClick,
  disabled
}: {
  children: JSX.Element | JSX.Element[] | string;
  className?: string;
  onClick?: () => void;
  disabled?: boolean;
}) => {
  return (
    <button
      className={`bg-none hover:bg-secondary px-4 py-2 text-white hover:text-black border border-secondary rounded-md disabled:pointer-events-none ${className}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
};
