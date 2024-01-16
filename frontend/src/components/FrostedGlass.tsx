type frostedWithTheme = {
  children: JSX.Element | JSX.Element[] | string
  className?: string
  theme?: 'light' | 'dark'
}

export const FrostedGlass = ({children, className, theme}: frostedWithTheme) => {
  if (theme === 'dark')
    return (
      <div className={`rounded-3xl p-xl backdrop-blur-md bg-gradient-to-br from-white/[.7] to-white/[0.4] ${className}`}>
        {children}
      </div>
    );
  else return (
    <div className={`rounded-3xl p-xl backdrop-blur-md bg-gradient-to-br from-black/[.7] to-black/[0.4] ${className}`}>
      {children}
    </div>
  );
};
