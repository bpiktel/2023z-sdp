import { Stage } from './components/stage'

const Home = (): JSX.Element => {
  return (
    <main className="flex">
      <div className="w-full h-screen">
        <Stage />
      </div>
    </main>
  )
}

export default Home
