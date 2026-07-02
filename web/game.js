import init, { WasmGame } from './pkg/solitaire_wasm.js';

// Constants
let CARD_W = 40;
let CARD_H = 66;
let PILE_SPACING = 92;
const MARGIN = 30;
const TABLEAU_Y = 240;
let TABLEAU_CARD_OFFSET = 25;
const TOP_ROW_Y = 30;
const RESTART_BTN = { x: 0, y: 0, w: 130, h: 48 };
let dragOffsetX = 0;
let dragOffsetY = 0;

// State
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const cardImages = {};
/** @type {import('./pkg/solitaire_wasm.js').WasmGame} */
let game;
let gameWon = false;
let selected = null;

// Image loading
function loadImage(src) {
	return new Promise((resolve) => {
		const img = new Image();
		img.onload = () => resolve(img);
		img.src = src;
	});
}

async function loadImages() {
	const suits = ['H', 'D', 'C', 'S'];
	const promises = [];
	for (const suit of suits) {
		for (let value = 1; value <= 13; value++) {
			const key = `${value}${suit}`;
			promises.push(
				loadImage(`./Assets/Sprites/${key}.png`).then((img) => {
					cardImages[key] = img;
				}),
			);
		}
	}
	promises.push(
		loadImage('./Assets/Sprites/CardBackRed.png').then((img) => {
			cardImages['back'] = img;
		}),
	);
	promises.push(
		loadImage('./Assets/Sprites/GreenReload.png').then((img) => {
			cardImages['recycle'] = img;
		}),
	);
	await Promise.all(promises);
}

// Helpers
function cardKey(card) {
	const suitMap = { Hearts: 'H', Diamonds: 'D', Clubs: 'C', Spades: 'S' };
	return `${card.value}${suitMap[card.suit]}`;
}

function drawCard(ctx, card, x, y, highlight = false) {
	const img = card.face_up ? cardImages[cardKey(card)] : cardImages['back'];
	if (highlight) {
		ctx.drawImage(img, x, y, CARD_W, CARD_H);
		/*
		ctx.save();
		ctx.strokeStyle = 'yellow';
		ctx.lineWidth = 2;
		ctx.strokeRect(x + 1, y + 1, CARD_W - 2, CARD_H - 2);
		ctx.restore();
        */
	} else {
		ctx.drawImage(img, x, y, CARD_W, CARD_H);
	}
}

// Rendering
function drawState(state, mouse_x, mouse_y) {
	ctx.clearRect(0, 0, canvas.width, canvas.height);

	// Calculate dynamic offset so tallest pile fits on screen
	const maxPileSize = Math.max(...state.tableau.map((p) => p.length), 1);
	const availableHeight = canvas.height - TABLEAU_Y - CARD_H - MARGIN;
	const dynamicOffset =
		maxPileSize > 1
			? Math.min(
					TABLEAU_CARD_OFFSET,
					Math.floor(availableHeight / (maxPileSize - 1)),
				)
			: TABLEAU_CARD_OFFSET;

	const draggedCards = [];
	// Deck
	if (state.deck_size > 0)
		ctx.drawImage(cardImages['back'], MARGIN, TOP_ROW_Y, CARD_W, CARD_H);
	else if (state.waste.length > 0)
		ctx.drawImage(cardImages['recycle'], MARGIN, TOP_ROW_Y, CARD_W, CARD_H);

	// Waste
	if (state.waste.length > 0) {
		const isSelected = selected?.kind === 'waste';
		if (!isSelected) {
			drawCard(
				ctx,
				state.waste[state.waste.length - 1],
				MARGIN + PILE_SPACING,
				TOP_ROW_Y,
				isSelected,
			);
		} else {
			draggedCards.push({
				card: state.waste[state.waste.length - 1],
				x: mouse_x - dragOffsetX,
				y: mouse_y - dragOffsetY,
				highlight: true,
			});
			if (state.waste.length > 1) {
				drawCard(
					ctx,
					state.waste[state.waste.length - 2],
					MARGIN + PILE_SPACING,
					TOP_ROW_Y,
					false,
				);
			}
		}
	}

	state.foundation.forEach((pile, i) => {
		if (pile.length > 0)
			drawCard(
				ctx,
				pile[pile.length - 1],
				MARGIN + (3 + i) * PILE_SPACING,
				TOP_ROW_Y,
			);
		else {
			ctx.save();
			ctx.strokeStyle = 'rgba(255,255,255,0.3)';
			ctx.lineWidth = 1;
			ctx.strokeRect(
				MARGIN + (3 + i) * PILE_SPACING + 0.5,
				TOP_ROW_Y + 0.5,
				CARD_W - 1,
				CARD_H - 1,
			);
			ctx.restore();
		}
	});

	state.tableau.forEach((pile, col) => {
		const cx = MARGIN + col * PILE_SPACING;

		if (pile.length === 0) {
			ctx.save();
			ctx.strokeStyle = 'rgba(255,255,255,0.3)';
			ctx.lineWidth = 1;
			ctx.strokeRect(cx + 0.5, TABLEAU_Y + 0.5, CARD_W - 1, CARD_H - 1);
			ctx.restore();
			return;
		}

		pile.forEach((card, row) => {
			const isSelected =
				selected?.kind === 'tableau' &&
				selected.col === col &&
				row >= selected.row;
			if (!isSelected) {
				drawCard(
					ctx,
					card,
					MARGIN + col * PILE_SPACING,
					TABLEAU_Y + row * dynamicOffset,
					isSelected,
				);
			} else {
				draggedCards.push({
					card: card,
					x: mouse_x - dragOffsetX,
					y:
						mouse_y +
						(row - selected.row) * dynamicOffset -
						dragOffsetY,
					highlight: true,
				});
			}
		});
	});
	drawRestartButton();
	for (const dragged of draggedCards) {
		drawCard(ctx, dragged.card, dragged.x, dragged.y, dragged.highlight);
	}
	return dynamicOffset;
}

function render(mouse_x = null, mouse_y = null) {
	const state = game.get_state();
	const dynamicOffset = drawState(state, mouse_x, mouse_y);
	gameWon = game.is_game_won();
	if (gameWon) drawWinScreen();
	return { state, dynamicOffset };
}

// Input
//
canvas.addEventListener('contextmenu', (e) => {
	e.preventDefault();
	selected = null;
	render();
});

canvas.addEventListener('click', (e) => {
	const x = e.offsetX;
	const y = e.offsetY;
	if (gameWon) {
		game = new WasmGame();
		gameWon = false;
		selected = null;
		render();
		return;
	}
	const { x: bx, y: by, w: bw, h: bh } = RESTART_BTN;
	if (x >= bx && x <= bx + bw && y >= by && y <= by + bh) {
		game = new WasmGame();
		selected = null;
		render();
		return;
	}
	const { state, dynamicOffset } = render();
	// Deck click → draw to waste (or flush if empty)
	if (hitCard(x, y, MARGIN, TOP_ROW_Y)) {
		if (state.deck_size === 0) {
			game.flush_waste();
		} else {
			game.from_deck_to_waste();
		}
		selected = null;
		render();
		return;
	}

	const target = getCardAt(x, y, state, dynamicOffset);
	if (!target) {
		selected = null;
		render();
		return;
	}

	if (e.shiftKey) {
		selected = null;
		if (e.altKey) {
			if (target.kind === 'waste') {
				game.auto_move_waste_to_tableau();
			} else if (target.kind === 'tableau') {
				const state = game.get_state();
				const count = state.tableau[target.col].length - target.row;
				game.auto_move_tableau_to_tableau(target.col, count);
			}
		} else {
			if (target.kind === 'waste') game.auto_move_waste_to_foundation();
			else if (target.kind === 'tableau')
				game.auto_move_tableau_to_foundation(target.col);
		}
		render();
		return;
	}

	if (!selected) {
		if (target.kind === 'foundation') return;
		selected = target;
		dragOffsetX = x - target.x;
		dragOffsetY = y - target.y;
	} else {
		attemptMoveFromSelected(target);
	}

	render(x, y);
});

canvas.addEventListener('mousemove', (e) => {
	if (!selected) return;
	const x = e.offsetX;
	const y = e.offsetY;
	render(x, y);
});

function getCardAt(x, y, state, dynamicOffset) {
	// Check waste
	if (state.waste.length > 0) {
		if (hitCard(x, y, MARGIN + PILE_SPACING, TOP_ROW_Y))
			return { kind: 'waste', x: MARGIN + PILE_SPACING, y: TOP_ROW_Y };
	}

	// Check tableau columns (iterate in reverse so top card wins)
	for (let col = 0; col < state.tableau.length; col++) {
		const pile = state.tableau[col];
		const cx = MARGIN + col * PILE_SPACING;

		if (pile.length === 0) {
			if (hitCard(x, y, cx, TABLEAU_Y)) {
				return { kind: 'tableau', col, row: 0, x: cx, y: TABLEAU_Y };
			}
			continue;
		}
		for (let row = pile.length - 1; row >= 0; row--) {
			const cy = TABLEAU_Y + row * dynamicOffset;
			// Only the last card has full height, others are clipped by overlap
			const h = row === pile.length - 1 ? CARD_H : TABLEAU_CARD_OFFSET;
			if (x >= cx && x <= cx + CARD_W && y >= cy && y <= cy + h) {
				if (pile[row].face_up)
					return { kind: 'tableau', col, row, x: cx, y: cy };
				else return null;
			}
		}
	}

	// Check foundations
	for (let i = 0; i < 4; i++) {
		if (hitCard(x, y, MARGIN + (3 + i) * PILE_SPACING, TOP_ROW_Y))
			return { kind: 'foundation', index: i };
	}

	return null;
}

function hitCard(x, y, cx, cy) {
	return x >= cx && x <= cx + CARD_W && y >= cy && y <= cy + CARD_H;
}

function attemptMoveFromSelected(target) {
	if (!selected) return;

	if (selected.kind === 'waste') {
		if (target.kind === 'tableau')
			game.attempt_move_waste_to_tableau(target.col);
		else if (target.kind === 'foundation')
			game.attempt_move_waste_to_foundation(target.index);
	} else if (selected.kind === 'tableau') {
		const state = game.get_state();
		const pile = state.tableau[selected.col];
		const count = pile.length - selected.row;

		if (target.kind === 'tableau') {
			game.attempt_move_tableau_to_tableau(
				selected.col,
				target.col,
				count,
			);
		} else if (target.kind === 'foundation') {
			if (count === 1)
				game.attempt_move_tableau_to_foundation(
					selected.col,
					target.index,
				);
		}
	}

	selected = null;
}

function drawRestartButton() {
	const { x, y, w, h } = RESTART_BTN;
	ctx.fillStyle = 'rgba(0,0,0,0.45)';
	ctx.beginPath();
	ctx.roundRect(x, y, w, h, 6);
	ctx.fill();
	ctx.strokeStyle = 'rgba(255,255,255,0.35)';
	ctx.lineWidth = 1;
	ctx.stroke();
	ctx.fillStyle = 'white';
	ctx.font = `bold ${Math.floor(CARD_W * 0.22)}px sans-serif`;
	ctx.textAlign = 'center';
	ctx.textBaseline = 'middle';
	ctx.fillText('↺ Restart', x + w / 2, y + h / 2);
}
function drawWinScreen() {
	// Dim overlay
	ctx.fillStyle = 'rgba(0,0,0,0.6)';
	ctx.fillRect(0, 0, canvas.width, canvas.height);

	// Card
	const bw = 320,
		bh = 200;
	const bx = (canvas.width - bw) / 2;
	const by = (canvas.height - bh) / 2;
	ctx.fillStyle = '#1a472a';
	ctx.beginPath();
	ctx.roundRect(bx, by, bw, bh, 12);
	ctx.fill();
	ctx.strokeStyle = 'gold';
	ctx.lineWidth = 2;
	ctx.stroke();

	// Text
	ctx.fillStyle = 'gold';
	ctx.font = 'bold 42px sans-serif';
	ctx.textAlign = 'center';
	ctx.textBaseline = 'middle';
	ctx.fillText('🎉 You Win!', canvas.width / 2, by + 70);

	// Restart button inside win screen
	ctx.fillStyle = 'white';
	ctx.font = 'bold 18px sans-serif';
	ctx.fillText('Click anywhere to restart', canvas.width / 2, by + 140);
}

function resizeCanvas() {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	// Fit 7 columns across the full width
	PILE_SPACING = Math.floor((canvas.width - MARGIN * 16) / 7);
	CARD_W = Math.floor(PILE_SPACING - 8); // small gap between columns
	CARD_H = Math.floor(CARD_W * 1.5);
	TABLEAU_CARD_OFFSET = Math.floor(CARD_H * 0.25);
	RESTART_BTN.x = canvas.width - MARGIN - RESTART_BTN.w;
	RESTART_BTN.y = TOP_ROW_Y + CARD_H + 10;
}
// Entry Point
async function main() {
	await init('./pkg/solitaire_wasm_bg.wasm');
	game = new WasmGame();

	await loadImages();
	window.addEventListener('resize', () => {
		resizeCanvas();
		render();
	});
	resizeCanvas();
	render();
}

main();
