class LinesCell extends React.PureComponent {
  render() {
    return <td className={`lines-board-cell color-${this.props.cell}`}><div className="ball" /></td>
  }
}

class LinesBoard extends React.PureComponent {
  render() {
    return <table class="lines-board"><tbody>{this.props.board.map((line, index) => <tr className="lines-board-line" key={index}>{line.map((cell, index) => <LinesCell cell={cell} key={index} />)}</tr>)}</tbody></table>
  }
}

class LinesGameLogViewer extends React.PureComponent {
  state = { step: 0 }

  componentDidMount() {
    this.timer = setInterval(() => this.setState({ step: this.state.step + 1 }), 1000);
  }

  componentWillUnmount() {
    clearInterval(this.timer)
  }

  static getDerivedStateFromProps(nextProps, prevState) {
    return {...LinesGameLogViewer.initialState}
  }

  render() {
    const currentGame = this.props.gameLog[Math.min(this.state.step, this.props.gameLog.length - 1)]
    return <React.Fragment>
      <div className="lines-score">{currentGame.score}</div>
      <LinesBoard board={currentGame.board} />
      {this.state.step >= this.props.gameLog.length && <h1>{`The game is over!`}</h1>}
    </React.Fragment>
  }
}

export default class extends React.Component {
  state = { gameLog: [] }

  setgameLog = (event) => {
    const gameLogLines = event.target.value.split('\n')
    let gameLog = []
    let gameStartIndex = 0
    while (gameStartIndex < gameLogLines.length) {
      const gameState = gameLogLines.slice(gameStartIndex, gameStartIndex + 12)
      gameStartIndex += 12
      const score = gameState[10]
      const board = gameState.slice(0, 9).map(line => line.split(' '))
      gameLog.push({ board, score })
      if (!gameState[11]) break;
      const [from_x, from_y, to_x, to_y] = gameState[11].split(' ')
      console.log(gameState, from_x, from_y)
      const boardWithMove = board.map(line => [...line])
      boardWithMove[to_y - 1][to_x - 1] = boardWithMove[from_y - 1][from_x - 1]
      boardWithMove[from_y - 1][from_x - 1] = '_'
      gameLog.push({ board: boardWithMove, score })
    }
    this.setState({ gameLog })
  }

  render() {
    return <React.Fragment>
      <style>{`
        .lines-game {
          background: url('/static/screen.png') no-repeat;
          width: 1594px;
          height: 874px;
        }
        .lines-score {
          color: white;
          font-size: 20px;
          font-family: monospace;
          float: right;
          padding: 28px 150px 0 0;
        }
        .lines-board {
          padding: 124px 0 0 423px;
        }
        .lines-board-line {
          height: 50px;
        }
        .lines-board-cell {
          width: 65px;
          border: 1px solid #333;
          padding: 3px 8px;
        }
        .lines-board-cell .ball { width: 65px; height: 50px; }
        .lines-board-cell.color-R .ball { background: url('/static/red.png') no-repeat; }
        .lines-board-cell.color-G .ball { background: url('/static/green.png') no-repeat; }
        .lines-board-cell.color-B .ball { background: url('/static/blue.png') no-repeat; }
        .lines-board-cell.color-M .ball { background: url('/static/maroon.png') no-repeat; }
        .lines-board-cell.color-P .ball { background: url('/static/pink.png') no-repeat; }
        .lines-board-cell.color-C .ball { background: url('/static/cyan.png') no-repeat; }
        .lines-board-cell.color-Y .ball { background: url('/static/yellow.png') no-repeat; }
      `}</style>
      <h1>Lines</h1>
      <textarea onChange={this.setgameLog} />
      <div className="lines-game">
        {this.state.gameLog.length > 0 && <LinesGameLogViewer gameLog={this.state.gameLog}/>}
      </div>
    </React.Fragment>
  }
}
