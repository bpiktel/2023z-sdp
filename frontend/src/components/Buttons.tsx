export const ButtonPrimary = ({
  children,
  className,
  onClick
}: {
  children: JSX.Element | JSX.Element[] | string;
  className: string;
  onClick: () => void;
}) => {
  return (
    <button
      className={`bg-primary hover:bg-primary-dark px-4 py-2 text-white rounded-md ${className}`}
      onClick={onClick}
    >
      {children}
    </button>
  );
};
